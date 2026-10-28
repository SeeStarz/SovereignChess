from PySide6.QtWidgets import QWidget, QVBoxLayout, QPushButton
from PySide6.QtGui import QIcon, QPixmap
from PySide6.QtCore import Slot, Signal, QSize, Qt, QPoint
from adapter_py.model.engine import PieceType, PieceSimple, FactionId
from frontend_pyside6.helper import piece_image
from frontend_pyside6.setting import SettingLoader


class PromotionWidget(QWidget):
    widget_map: dict[PieceType, QPushButton]
    on_promotion_selected: Signal = Signal(PieceType)

    def __init__(self):
        super().__init__()

        vertical = QVBoxLayout()
        vertical.setAlignment(Qt.AlignmentFlag.AlignTop)
        vertical.setContentsMargins(0, 0, 0, 0)
        vertical.setSpacing(0)

        self.widget_map = {}
        for piece_type in ("King", "Queen", "Rook", "Bishop", "Knight"):
            piece_type = PieceType(piece_type)

            tile_size = QSize(
                SettingLoader.get().tile_size, SettingLoader.get().tile_size
            )

            button = QPushButton()
            button.setIconSize(tile_size)
            button.setFixedSize(tile_size)
            button.pressed.connect(
                lambda p=piece_type: self.on_promotion_selected.emit(p)
            )

            self.widget_map[piece_type] = button
            vertical.addWidget(button)

        self.setLayout(vertical)

    @Slot(list)
    def update_active_promotions(self, active_promotions: list[PieceType]):
        self.setVisible(len(active_promotions) > 0)
        for piece_type, button in self.widget_map.items():
            button.setVisible(piece_type in active_promotions)

    @Slot(FactionId, FactionId)
    def update_faction_and_owner(self, faction: FactionId, owner: FactionId):
        for piece_type, button in self.widget_map.items():
            piece = PieceSimple(
                faction=faction,
                piece_type=piece_type,
            )
            image = piece_image(piece, owner)

            tile_size = QSize(
                SettingLoader.get().tile_size, SettingLoader.get().tile_size
            )

            pixmap = QPixmap.fromImage(image)
            pixmap = pixmap.scaled(tile_size)
            icon = QIcon(pixmap)

            button.setIcon(icon)

    @Slot(QPoint)
    def update_promotion_location(self, point: QPoint):
        self.move(point)
