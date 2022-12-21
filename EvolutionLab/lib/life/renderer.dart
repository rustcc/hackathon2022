import 'package:flutter/material.dart';

import '../bridge/bridge.dart';
import '../bridge/bridge_extension.dart';

class LifeRenderer extends StatelessWidget {
  const LifeRenderer(this.shape, this.cells, {super.key});

  final ValueNotifier<Shape> shape;
  final ValueNotifier<List<Position>> cells;

  @override
  Widget build(BuildContext context) {
    final screenSize = MediaQuery.of(context).size;

    return ValueListenableBuilder(
        valueListenable: shape,
        builder: (context, value, _) {
          final canvasSize = value.getCanvasSize(screenSize);

          return CustomPaint(
            size: canvasSize,
            painter: _CellPainter(cells, canvasSize.height / value.y),
            child: RepaintBoundary(
              child: CustomPaint(size: canvasSize, painter: _GridPainter(value)),
            ),
          );
        });
  }
}

class _CellPainter extends CustomPainter {
  final double cellWidth;
  final ValueNotifier<List<Position>> cells;

  _CellPainter(this.cells, this.cellWidth) : super(repaint: cells);

  final cellPaint = Paint()
    ..isAntiAlias = true
    ..color = Colors.black
    ..style = PaintingStyle.fill;

  @override
  void paint(Canvas canvas, Size size) {
    var path = Path();

    for (final c in cells.value) {
      path.addRect(c.toRect(cellWidth));
    }

    canvas.drawPath(path, cellPaint);
  }

  @override
  bool shouldRepaint(_) => true;
}

class _GridPainter extends CustomPainter {
  final Shape shape;

  _GridPainter(this.shape);

  final gridPaint = Paint()
    ..isAntiAlias
    ..color = const Color.fromARGB(255, 210, 210, 210)
    ..style = PaintingStyle.stroke;

  @override
  void paint(Canvas canvas, Size size) {
    final width = size.height / shape.y;

    for (int x = 0; x <= shape.x; x++) {
      canvas.drawLine(Offset(x * width, 0), Offset(x * width, size.height), gridPaint);
    }

    for (var i = 0; i <= shape.y; i++) {
      canvas.drawLine(Offset(0, i * width), Offset(size.width, i * width), gridPaint);
    }
  }

  @override
  bool shouldRepaint(_) => true;
}
