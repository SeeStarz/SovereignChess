from PySide6.QtGui import QPainter, QColor, QImage
from PySide6.QtWidgets import QStackedWidget, QWidget
from adapter_py.model.engine import (
    ColorStandard,
    FactionId,
    PieceSimple,
)
from typing import Optional


def piece_image(piece: PieceSimple, owner: Optional[FactionId]) -> QImage:
    top_asset_path = "assets/img/{}.png".format(piece.piece_type.root.lower())
    base_asset_path = "assets/img/{}_base.png".format(piece.piece_type.root.lower())
    neutral_base_asset_path = "assets/img/{}_neutral.png".format(
        piece.piece_type.root.lower()
    )

    image = QImage(top_asset_path)
    top_color = faction_to_color(piece.faction)
    image = tint_image(image, top_color)

    with QPainter(image) as painter:
        if owner is None:
            base_image = QImage(neutral_base_asset_path)
            painter.drawImage(image.rect(), base_image)
        else:
            base_image = QImage(base_asset_path)
            base_color = faction_to_color(owner)
            base_image = tint_image(base_image, base_color)
            painter.drawImage(image.rect(), base_image)

    return image


def tint_image(image: QImage, color: QColor) -> QImage:
    image = image.convertToFormat(QImage.Format.Format_ARGB32_Premultiplied)
    mask = image.convertToFormat(QImage.Format.Format_Alpha8)

    with QPainter(image) as painter:
        painter.setCompositionMode(QPainter.CompositionMode.CompositionMode_Multiply)
        painter.fillRect(image.rect(), color)
        painter.setCompositionMode(
            QPainter.CompositionMode.CompositionMode_DestinationIn
        )
        painter.drawImage(image.rect(), mask)

    return image


def faction_to_color(faction_id: FactionId) -> QColor:
    DEFAULT = {
        ColorStandard.WHITE: (255, 255, 255),
        ColorStandard.PINK: (223, 159, 223),
        ColorStandard.SLATE: (127, 127, 127),
        ColorStandard.RED: (191, 31, 31),
        ColorStandard.ORANGE: (223, 95, 31),
        ColorStandard.YELLOW: (255, 255, 31),
        ColorStandard.GREEN: (31, 159, 31),
        ColorStandard.CYAN: (31, 191, 191),
        ColorStandard.NAVY: (31, 31, 191),
        ColorStandard.ASH: (207, 207, 207),
        ColorStandard.VIOLET: (159, 0, 233),
        ColorStandard.BLACK: (63, 63, 63),
    }

    color = DEFAULT.get(ColorStandard(faction_id.root))
    if color is None:
        print(f"Faction id {faction_id} does not have a color")
        return QColor.fromRgb(255, 0, 0)

    return QColor.fromRgb(*color)


def push_stack(stack: QStackedWidget, widget: QWidget):
    stack.addWidget(widget)
    stack.setCurrentWidget(widget)


def pop_stack(stack: QStackedWidget):
    widget = stack.currentWidget()
    assert widget is not None
    stack.removeWidget(widget)
    widget.deleteLater()
    stack.setCurrentIndex(stack.count() - 1)
