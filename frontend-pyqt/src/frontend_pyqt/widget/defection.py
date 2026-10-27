from PySide6.QtWidgets import QWidget, QGridLayout, QPushButton
from PySide6.QtGui import QIcon, QPixmap
from PySide6.QtCore import Slot, Signal, QSize, Qt
from adapter_py.model.engine import PieceType, PieceSimple, FactionId
from frontend_pyqt.helper import piece_image
from frontend_pyqt.setting import SettingLoader


class DefectionWidget(QWidget):
    widget_map: dict[FactionId, QWidget]
    on_defection_selected: Signal = Signal(PieceType)

    def __init__(self, faction_list: list[FactionId]):
        super().__init__()

        layout = QGridLayout()
        layout.setSpacing(SettingLoader.get().close_spacing)
        layout.setAlignment(Qt.AlignmentFlag.AlignCenter)
        layout.setContentsMargins(0, 0, 0, 0)

        self.widget_map = {}
        for i, faction in enumerate(sorted(faction_list, key=lambda f: f.root)):
            piece = PieceSimple(
                faction=faction,
                piece_type=PieceType(root="King"),
            )
            image = piece_image(piece, faction)

            tile_size = QSize(
                SettingLoader.get().tile_size, SettingLoader.get().tile_size
            )

            pixmap = QPixmap.fromImage(image)
            pixmap = pixmap.scaled(tile_size)
            icon = QIcon(pixmap)

            button = QPushButton()

            button.setIcon(icon)
            button.setIconSize(tile_size)
            button.setFixedSize(tile_size + QSize(4, 4))

            button.pressed.connect(lambda f=faction: self.on_defection_selected.emit(f))

            self.widget_map[faction] = button

            modulo = (len(faction_list) + 1) // 2
            col = i % modulo
            row = i // modulo
            layout.addWidget(button, row, col)

        self.setLayout(layout)

    @Slot(list)
    def update_active_defections(self, active_defections: list[FactionId]):
        for faction, button in self.widget_map.items():
            button.setEnabled(faction in active_defections)
