from PySide6.QtWidgets import QWidget, QHBoxLayout, QPushButton
from PySide6.QtGui import QIcon, QPixmap
from PySide6.QtCore import Slot, Signal
from adapter_py.model.engine import PieceType, PieceSimple, FactionId
from frontend_pyqt.helper import piece_image


class DefectionWidget(QWidget):
    widget_map: dict[FactionId, QWidget]
    on_defection_selected: Signal = Signal(PieceType)

    def __init__(self, faction_list: list[FactionId]):
        super().__init__()

        layout = QHBoxLayout()

        self.widget_map = {}
        for faction in sorted(faction_list, key=lambda f: f.root):
            piece = PieceSimple(
                faction=faction,
                piece_type=PieceType(root="King"),
            )
            image = piece_image(piece, FactionId(root=0))
            icon = QIcon(QPixmap.fromImage(image))

            button = QPushButton()
            button.setIcon(icon)
            button.pressed.connect(lambda f=faction: self.on_defection_selected.emit(f))

            self.widget_map[faction] = button
            layout.addWidget(button)

        self.setLayout(layout)

    @Slot(list)
    def update_active_defections(self, active_defections: list[FactionId]):
        for faction, button in self.widget_map.items():
            button.setEnabled(faction in active_defections)
