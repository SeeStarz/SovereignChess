from PySide6.QtWidgets import QWidget, QGridLayout, QPushButton
from PySide6.QtGui import QIcon, QPixmap
from PySide6.QtCore import Slot, Signal, QSize
from adapter_py.model.engine import PieceType, PieceSimple, FactionId
from frontend_pyside6.helper import piece_image
from frontend_pyside6.setting import SettingLoader


class DefectionWidget(QWidget):
    widget_map: dict[FactionId, QPushButton]
    on_defection_selected: Signal = Signal(PieceType)
    on_deselect: Signal = Signal()

    def __init__(self, faction_list: list[FactionId]):
        super().__init__()

        layout = QGridLayout()
        layout.setSpacing(2)
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
            button.setCheckable(True)

            button.setIcon(icon)
            button.setIconSize(tile_size)
            button.setFixedSize(tile_size + QSize(4, 4))

            button.clicked.connect(
                lambda checked, b=button, f=faction: self.connect_checked(checked, b, f)
            )
            self.widget_map[faction] = button

            modulo = (len(faction_list) + 1) // 2
            col = i % modulo
            row = i // modulo
            layout.addWidget(button, row, col)

        self.setLayout(layout)

    @Slot()
    def deselect_all(self):
        for button in self.widget_map.values():
            button.setChecked(False)

    def deselect_others(self, button: QPushButton):
        for faction, mapped_button in self.widget_map.items():
            if mapped_button != button:
                mapped_button.setChecked(False)

    @Slot(list)
    def update_active_defections(self, active_defections: list[FactionId]):
        for faction, button in self.widget_map.items():
            button.setEnabled(faction in active_defections)

    def connect_checked(self, checked: bool, button: QPushButton, faction: FactionId):
        if checked:
            self.deselect_others(button)
            self.on_defection_selected.emit(faction)
        else:
            self.on_deselect.emit()
