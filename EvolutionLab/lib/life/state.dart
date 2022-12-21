import 'package:flutter/material.dart';
import 'package:shared_preferences/shared_preferences.dart';

import '../bridge/bridge.dart';
import '../assets/pattern.dart';
import '../bridge/bridge_extension.dart';

class LifeState {
  late Life _life;

  final cells = ValueNotifier<List<Position>>([]);
  final shape = ValueNotifier<Shape>(Shape(x: 100, y: 50));
  late PatternCollectList patternCollectList;
  late SharedPreferences _sharedPreferences;

  Boundary _boundary = Boundary.None;
  Boundary get boundary => _boundary;

  Future<void> initState() async {
    patternCollectList = PatternCollectList();
    patternCollectList.init();
    _sharedPreferences = await SharedPreferences.getInstance();

    final prevBoundary = _sharedPreferences.getString('boundary');
    _boundary = _boundary.fromName(prevBoundary);

    final prevPattern = _sharedPreferences.getString('prev_pattern');
    _sharedPreferences.remove('prev_pattern');

    Pattern pattern;
    if (prevPattern == null) {
      pattern = await bridge.defaultPattern();
    } else {
      try {
        pattern = await bridge.decodeRle(rle: prevPattern);
        shape.value = Shape(x: pattern.header.x, y: pattern.header.y);
      } catch (_) {
        pattern = await bridge.defaultPattern();
      }
    }

    cells.value = pattern.cells;
    _life = await bridge.create(shape: shape.value, boundary: _boundary);
    await _life.setCells(cells: pattern.cells);
  }

  Future<bool> saveState() async {
    return _sharedPreferences.setString(
      'prev_pattern',
      await bridge.encodeRle(
        header: Header(x: shape.value.x, y: shape.value.y),
        cells: cells.value,
      ),
    );
  }

  /*
  * 控制生命演化的方法
  */
  var isPaused = true;
  var _delayed = const Duration(milliseconds: 100);
  late final _controller = _createEvolveStream().listen(null);

  Stream<void> _createEvolveStream() async* {
    while (true) {
      await Future.wait([_life.evolve(), Future.delayed(_delayed)]);
      cells.value = await _life.getCells();

      yield null;
    }
  }

  void pause() {
    if (!isPaused) {
      _controller.pause();
      isPaused = _controller.isPaused;
    }
  }

  void resume() {
    _controller.resume();
    isPaused = _controller.isPaused;
  }

  void setDelayed(int milliseconds) {
    _delayed = Duration(milliseconds: milliseconds);
  }

  /*
  * 重新包装的 Rust 端 API
  */
  void next() async {
    await _life.evolve();
    cells.value = await _life.getCells();
  }

  Future<void> rand(double distr) async {
    await _life.rand(distr: distr);
    cells.value = await _life.getCells();
  }

  Future<void> setShape(Shape newShape, {bool? clean}) async {
    await _life.setShape(shape: newShape, clean: clean);

    if (clean == true) {
      cells.value = [];
    } else {
      cells.value = await _life.getCells();
    }

    shape.value = newShape;
  }

  Future<void> setCells(List<Position> newCells) async {
    await _life.setCells(cells: newCells);
    cells.value = await _life.getCells();
  }

  Future<void> cleanCells() => _life.cleanCells();

  Future<void> setBoundary(Boundary boundary) async {
    _life.setBoundary(boundary: boundary);
    _boundary = boundary;

    _sharedPreferences.setString('boundary', _boundary.name);
  }

  void dispose() {
    _controller.cancel();
    _life.field0.dispose();
  }
}
