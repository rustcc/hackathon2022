import 'package:flutter/material.dart';

import './bridge.dart';

extension ImplShape on Shape {
  Size getCanvasSize(Size screenSize) {
    final width = screenSize.width / x;
    final height = screenSize.height / y;
    final cellWidth = height < width ? height : width;

    return Size(x * cellWidth, y * cellWidth);
  }

  bool include(Shape shape) => x >= shape.x && y >= shape.y;

  Position getCenterOffset(Shape targetShape) =>
      Position(x: (targetShape.x - x) ~/ 2, y: (targetShape.y - y) ~/ 2);
}

extension ImplPosition on Position {
  Rect toRect(double width) => Rect.fromLTWH(x * width, y * width, width, width);
}

extension ImplHeader on Header {
  Shape getShape() => Shape(x: x, y: y);

  Widget toWidget({BuildContext? context}) => Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text('Name: $name'),
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              Text('Owner: $owner'),
              context != null && comment != null
                  ? TextButton(
                      child: const Text('more'),
                      onPressed: () => showDialog(
                        context: context,
                        builder: (_) => AlertDialog(
                          title: const Text('更多信息'),
                          content: SelectableText(comment!),
                        ),
                      ),
                    )
                  : Container(),
            ],
          ),
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [Text('Size: ${x}x$y'), Text('Rule: $rule')],
          ),
        ],
      );
}

extension ImplPattern on Pattern {
  Pattern applyOffset(Position offset) {
    return Pattern(
      header: Header(
        name: header.name,
        owner: header.owner,
        comment: header.comment,
        rule: header.rule,
        x: header.x + offset.x,
        y: header.y + offset.y,
      ),
      cells: cells.map((e) => Position(x: e.x + offset.x, y: e.y + offset.y)).toList(),
    );
  }
}

extension ImplBoundary on Boundary {
  Boundary fromName(String? name) =>
      {
        Boundary.None.name: Boundary.None,
        Boundary.Sphere.name: Boundary.Sphere,
        Boundary.Mirror.name: Boundary.Mirror,
      }[name] ??
      this;
}
