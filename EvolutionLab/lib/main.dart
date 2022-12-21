import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

import './life/state.dart';
import './life/renderer.dart';
import './ui/button.dart';
import './ui/utils.dart';
import './ui/drawer.dart';

void main() async {
  // 横屏，去状态栏
  WidgetsFlutterBinding.ensureInitialized();
  SystemChrome.setPreferredOrientations([
    DeviceOrientation.landscapeLeft,
    DeviceOrientation.landscapeRight,
  ]);
  SystemChrome.setEnabledSystemUIMode(SystemUiMode.manual, overlays: []);

  final life = LifeState();
  await life.initState();

  runApp(MaterialApp(
    home: EvolutionLab(life),
    theme: ThemeData(
      floatingActionButtonTheme: FloatingActionButtonThemeData(
        elevation: 0,
        backgroundColor: Colors.blue.withAlpha(150),
      ),
      sliderTheme: const SliderThemeData(
        showValueIndicator: ShowValueIndicator.never,
        thumbShape: ThumbShape(),
      ),
    ),
  ));
}

class EvolutionLab extends StatefulWidget {
  const EvolutionLab(this.life, {super.key});

  final LifeState life;

  @override
  State<EvolutionLab> createState() => _EvolutionLabState();
}

class _EvolutionLabState extends State<EvolutionLab> with WidgetsBindingObserver {
  LifeState get life => widget.life;

  @override
  Widget build(BuildContext context) {
    return SaveDialog(
      life: life,
      child: Scaffold(
        drawerEdgeDragWidth: 0,
        endDrawer: Drawer(child: EndDrawer(life)),
        body: InteractiveViewer(
          maxScale: 1000,
          child: Center(
            child: LifeRenderer(life.shape, life.cells),
          ),
        ),
        floatingActionButton: ControllerButton(life),
        resizeToAvoidBottomInset: false,
      ),
    );
  }

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addObserver(this);
  }

  @override
  void dispose() {
    life.dispose();
    super.dispose();
  }

  @override
  void didChangeAppLifecycleState(AppLifecycleState state) {
    switch (state) {
      case AppLifecycleState.paused:
        life.pause(); // 后台时暂停
        break;
      case AppLifecycleState.resumed:
        setState(() {});
        break;
      default:
        break;
    }
  }
}
