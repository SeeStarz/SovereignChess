from PySide6.QtWidgets import QWidget, QHBoxLayout, QPushButton
from PySide6.QtGui import QIcon, QPixmap
from PySide6.QtCore import Slot, Signal
from adapter_py.model.engine import PieceType, PieceSimple, FactionId
from frontend_pyqt.helper import piece_image


class PromotionWidget(QWidget):
    widget_map: dict[PieceType, QWidget]
    on_promotion_selected: Signal = Signal(PieceType)

    def __init__(self):
        super().__init__()

        layout = QHBoxLayout()

        self.widget_map = {}
        for piece_type in ("King", "Queen", "Rook", "Bishop", "Knight"):
            piece_type = PieceType(piece_type)
            piece = PieceSimple(
                faction=FactionId(root=0),
                piece_type=piece_type,
            )
            image = piece_image(piece, FactionId(root=0))
            icon = QIcon(QPixmap.fromImage(image))

            button = QPushButton()
            button.setIcon(icon)
            button.pressed.connect(
                lambda p=piece_type: self.on_promotion_selected.emit(p)
            )

            self.widget_map[piece_type] = button
            layout.addWidget(button)

        self.setLayout(layout)

    @Slot(list)
    def update_active_promotions(self, active_promotions: list[PieceType]):
        for piece_type, button in self.widget_map.items():
            button.setEnabled(piece_type in active_promotions)
