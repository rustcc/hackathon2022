// ignore_for_file: use_build_context_synchronously

import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:file_picker/file_picker.dart';
import 'package:flutter_window_close/flutter_window_close.dart';

import '../life/state.dart';
import '../bridge/bridge.dart';
import '../bridge/bridge_extension.dart';

// 重设网格形状的弹窗
class ResetShapeDialog extends StatefulWidget {
  const ResetShapeDialog(
    this.life, {
    required this.targetShape,
    this.clean = true,
    this.title = '新建',
    this.moreTarget = false,
    super.key,
  });

  // 清除现有细胞
  final bool clean;
  // 弹窗标题
  final String title;
  final LifeState life;
  // 目标形状
  final Shape targetShape;
  // 如果为真，必需大于目标形状
  final bool moreTarget;

  @override
  State<ResetShapeDialog> createState() => _ResetShapeDialogState();
}

class _ResetShapeDialogState extends State<ResetShapeDialog> {
  String get title => widget.title;
  LifeState get life => widget.life;
  bool get moreTarget => widget.moreTarget;
  Shape get targetShape => widget.targetShape;

  bool fullScreen = true;
  late bool cleanCell = widget.clean;
  late Size size = MediaQuery.of(context).size;
  late TextEditingController widthCtrl = TextEditingController(text: targetShape.x.toString());
  late TextEditingController heightCtrl = TextEditingController(text: targetShape.y.toString());

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: Text(title),
      // 套一层 SingleChildScrollView 防止弹出键盘时遮盖输入框
      content: SingleChildScrollView(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.end,
          children: [
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                const Expanded(flex: 2, child: Text('大小', textAlign: TextAlign.start)),
                Expanded(
                  child: TextField(
                    autofocus: true,
                    controller: widthCtrl,
                    keyboardType: TextInputType.number,
                    inputFormatters: [FilteringTextInputFormatter.digitsOnly],
                    onChanged: (v) {
                      if (fullScreen) {
                        final width = int.tryParse(v) ?? 0;
                        final height = size.height ~/ (size.width / width);
                        heightCtrl.text = height.toString();
                      }
                    },
                  ),
                ),
                const Expanded(child: Text('x', textAlign: TextAlign.center)),
                Expanded(
                  child: TextField(
                    controller: heightCtrl,
                    keyboardType: TextInputType.number,
                    inputFormatters: [FilteringTextInputFormatter.digitsOnly],
                    onChanged: (v) {
                      if (fullScreen) {
                        final height = int.tryParse(v) ?? 0;
                        final width = size.width ~/ (size.height / height);
                        widthCtrl.text = width.toString();
                      }
                    },
                  ),
                ),
              ],
            ),
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                const Text('清除细胞'),
                Switch(
                  value: cleanCell,
                  onChanged: (i) => setState(() => cleanCell = i),
                ),
                const Text('占满屏幕'),
                Switch(
                  value: fullScreen,
                  onChanged: (i) => setState(() => fullScreen = i),
                ),
              ],
            ),
            TextButton(
                child: const Text('确认'),
                onPressed: () async {
                  final shape = Shape(
                    x: int.tryParse(widthCtrl.text) ?? 0,
                    y: int.tryParse(heightCtrl.text) ?? 0,
                  );

                  if (moreTarget && !shape.include(targetShape)) {
                    await showDialog(
                      context: context,
                      builder: (_) => AlertDialog(
                        content: SelectableText('网格必需大于 ${targetShape.x}x${targetShape.y}!'),
                      ),
                    );
                  } else {
                    await life.setShape(shape, clean: cleanCell);
                    Navigator.pop(context);
                  }
                })
          ],
        ),
      ),
    );
  }
}

// 弹框显示 pattern 信息并设置到网格，成功返回 true
Future<bool?> showPatternInfo(BuildContext context, LifeState life, Pattern pattern) {
  var center = true;
  var cleanCell = true;
  final shape = pattern.header.getShape();
  final moreShape = !life.shape.value.include(shape);

  return showDialog(
    context: context,
    builder: (context) => AlertDialog(
      title: const Text('RLE 信息'),
      content: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          pattern.header.toWidget(context: context),
          StatefulBuilder(
            builder: (context, setState) => Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                const Text('清除网格'),
                Switch(
                  value: cleanCell,
                  onChanged: (i) => setState(() => cleanCell = i),
                ),
                const Text('居中'),
                Switch(
                  value: center,
                  onChanged: (i) => setState(() => center = i),
                ),
              ],
            ),
          ),
        ],
      ),
      actions: [
        TextButton(
          child: Text(moreShape ? '尺寸过大，点此扩大网格' : '确认'),
          onPressed: () async {
            if (moreShape) {
              await showDialog(
                context: context,
                builder: (context) => ResetShapeDialog(
                  life,
                  targetShape: shape,
                  clean: false,
                  title: '调整网格大小',
                  moreTarget: true,
                ),
              );
            }

            if (life.shape.value.include(shape)) {
              if (center) {
                final offset = shape.getCenterOffset(life.shape.value);
                pattern = pattern.applyOffset(offset);
              }

              if (cleanCell) await life.cleanCells();

              await life.setCells(pattern.cells);

              Navigator.of(context).pop(true);
            }
          },
        )
      ],
    ),
  );
}

// 从文件打开一个 RLE 文件，成功返回 true
Future<bool?> openRleFile(BuildContext context, LifeState life) async {
  const title = '请选择一个 RLE 文件';
  FilePickerResult? filePicker;

  if (Platform.isLinux || Platform.isMacOS || Platform.isWindows) {
    filePicker = await FilePicker.platform.pickFiles(
      dialogTitle: title,
      type: FileType.custom,
      allowedExtensions: ['rle'],
    );
  } else {
    // 移动端不支持 rle 扩展名
    filePicker = await FilePicker.platform.pickFiles(dialogTitle: title, type: FileType.any);
  }

  final path = filePicker?.files.single.path;
  if (path == null) return false;

  try {
    final str = await File(path).readAsString();
    var rle = await bridge.decodeRle(rle: str);

    return showPatternInfo(context, life, rle);
  } catch (e) {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('打开文件失败！'),
        content: Text(e.toString()),
      ),
    );

    return false;
  }
}

class SaveDialog extends StatelessWidget {
  const SaveDialog({required this.life, required this.child, super.key});

  final Widget child;
  final LifeState life;

  @override
  Widget build(BuildContext context) {
    Future<bool> showSaveDialog() async {
      return await showDialog(
          context: context,
          builder: (context) {
            return AlertDialog(title: const Text('保存当前状态?'), actions: [
              ElevatedButton(
                child: const Text('保存并退出'),
                onPressed: () async {
                  await life.saveState();
                  return Navigator.of(context).pop(true);
                },
              ),
              ElevatedButton(
                child: const Text('直接退出'),
                onPressed: () => Navigator.of(context).pop(true),
              ),
            ]);
          });
    }

    if (Platform.isLinux || Platform.isMacOS || Platform.isWindows) {
      FlutterWindowClose.setWindowShouldCloseHandler(showSaveDialog);

      return child;
    } else {
      return WillPopScope(
        onWillPop: showSaveDialog,
        child: child,
      );
    }
  }
}

/*
* 绘制滑块的同时绘制当前数值，无需滑动才显示。
* See https://github.com/flutter/flutter/issues/34704
*/
class ThumbShape extends RoundSliderThumbShape {
  final _indicatorShape = const PaddleSliderValueIndicatorShape();

  const ThumbShape();

  @override
  void paint(
    PaintingContext context,
    Offset center, {
    required Animation<double> activationAnimation,
    required Animation<double> enableAnimation,
    required bool isDiscrete,
    required TextPainter labelPainter,
    required RenderBox parentBox,
    required SliderThemeData sliderTheme,
    required TextDirection textDirection,
    required double value,
    required double textScaleFactor,
    required Size sizeWithOverflow,
  }) {
    super.paint(
      context,
      center,
      activationAnimation: activationAnimation,
      enableAnimation: enableAnimation,
      sliderTheme: sliderTheme,
      value: value,
      textScaleFactor: textScaleFactor,
      sizeWithOverflow: sizeWithOverflow,
      isDiscrete: isDiscrete,
      labelPainter: labelPainter,
      parentBox: parentBox,
      textDirection: textDirection,
    );

    _indicatorShape.paint(
      context,
      center,
      activationAnimation: const AlwaysStoppedAnimation(1),
      enableAnimation: enableAnimation,
      labelPainter: labelPainter,
      parentBox: parentBox,
      sliderTheme: sliderTheme,
      value: value,
      textScaleFactor: 0.7,
      sizeWithOverflow: sizeWithOverflow,
      isDiscrete: isDiscrete,
      textDirection: textDirection,
    );
  }
}
