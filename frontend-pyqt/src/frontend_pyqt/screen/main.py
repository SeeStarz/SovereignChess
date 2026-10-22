from PySide6.QtCore import Qt
from PySide6.QtWidgets import (
    QVBoxLayout,
    QLabel,
    QWidget,
    QStackedWidget,
    QPushButton,
)
from adapter_py.model.engine import VariantPreset
from frontend_pyqt.helper import push_stack
from frontend_pyqt import screen


def main_screen(stack: QStackedWidget):
    layout = QVBoxLayout()
    layout.setAlignment(Qt.AlignmentFlag.AlignCenter)

    title = QLabel("# Sovereign Chess", textFormat=Qt.TextFormat.MarkdownText)
    layout.addWidget(title)

    standard = QPushButton("Standard")
    standard.clicked.connect(
        lambda: push_stack(stack, screen.game_screen(stack, VariantPreset("Standard")))
    )
    layout.addWidget(standard)

    arena = QPushButton("Arena")
    arena.clicked.connect(
        lambda: push_stack(stack, screen.game_screen(stack, VariantPreset("Arena")))
    )
    layout.addWidget(arena)

    widget = QWidget()
    widget.setLayout(layout)

    return widget
