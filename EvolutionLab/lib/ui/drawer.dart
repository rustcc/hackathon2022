import 'package:flutter/material.dart';
import 'package:url_launcher/url_launcher.dart';

import './utils.dart';
import '../life/state.dart';
import '../bridge/bridge.dart';
import '../bridge/bridge_extension.dart';

class EndDrawer extends StatefulWidget {
  const EndDrawer(this.life, {super.key});

  final LifeState life;

  @override
  State<EndDrawer> createState() => _EndDrawerState();
}

class _EndDrawerState extends State<EndDrawer> {
  LifeState get life => widget.life;

  bool isSetting = true;
  final lifeWiki = Uri.parse('https://conwaylife.com/wiki/Main_Page');

  @override
  Widget build(BuildContext context) {
    return Column(
      mainAxisAlignment: MainAxisAlignment.spaceBetween,
      children: [
        Expanded(child: isSetting ? _Setting(life) : CollectList(life)),
        ButtonBar(
          children: [
            IconButton(
              tooltip: '设置',
              icon: const Icon(Icons.settings),
              onPressed: () => setState(() => isSetting = true),
            ),
            IconButton(
              tooltip: '收藏',
              icon: const Icon(Icons.star),
              onPressed: () => setState(() => isSetting = false),
            ),
            IconButton(
              tooltip: 'LifeWiki',
              icon: const Icon(Icons.info),
              onPressed: () async => await launchUrl(lifeWiki),
            )
          ],
        )
      ],
    );
  }
}

class _Setting extends StatelessWidget {
  const _Setting(this.life);

  final LifeState life;

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        MaterialButton(
          child: const Text('新建或扩展网格'),
          onPressed: () => showDialog(
            context: context,
            builder: (_) => ResetShapeDialog(
              life,
              title: '新建或扩展网格',
              targetShape: life.shape.value,
            ),
          ),
        ),
        MaterialButton(
          child: const Text('打开 RLE 文件'),
          onPressed: () async {
            if (true == await openRleFile(context, life)) {
              // ignore: use_build_context_synchronously
              Navigator.pop(context);
            }
          },
        ),
        SetBoundary(life),
      ],
    );
  }
}

class SetBoundary extends StatelessWidget {
  SetBoundary(this.life, {super.key});

  final LifeState life;

  final items = [Boundary.Sphere.name, Boundary.Mirror.name, Boundary.None.name]
      .map((n) => DropdownMenuItem(value: n, child: Text(n)))
      .toList();

  @override
  Widget build(BuildContext context) {
    return Row(
      children: [
        const Expanded(child: Text('边界条件:', textAlign: TextAlign.center, textScaleFactor: 1.1)),
        Expanded(
          child: DropdownButtonHideUnderline(
            child: StatefulBuilder(
              builder: (context, setState) => DropdownButton(
                items: items,
                isExpanded: true,
                value: life.boundary.name,
                onChanged: (String? value) async {
                  await life.setBoundary(life.boundary.fromName(value));
                  setState(() {});
                },
              ),
            ),
          ),
        ),
      ],
    );
  }
}

class CollectList extends StatelessWidget {
  const CollectList(this.life, {super.key});

  final LifeState life;

  @override
  Widget build(BuildContext context) {
    return ListView.separated(
      itemCount: life.patternCollectList.length(),
      separatorBuilder: (_, i) => const Divider(),
      itemBuilder: (context, index) => FutureBuilder(
        future: life.patternCollectList.getPattern(index),
        builder: (context, snapshot) {
          if (snapshot.connectionState == ConnectionState.done) {
            return Padding(
              padding: const EdgeInsets.all(8),
              child: GestureDetector(
                behavior: HitTestBehavior.opaque,
                child: snapshot.data!.header.toWidget(),
                onTap: () => showPatternInfo(context, life, snapshot.data!),
              ),
            );
          } else {
            return const CircularProgressIndicator();
          }
        },
      ),
    );
  }
}
