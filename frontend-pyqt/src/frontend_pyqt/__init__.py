from PySide6.QtWidgets import (
    QApplication,
    QVBoxLayout,
    QLabel,
    QWidget,
    QStackedWidget,
    QPushButton,
)
from PySide6.QtCore import Qt, QTimer
from adapter_py.api.session import SessionAdapter
from adapter_py.model.engine import VariantPreset
import sys
import signal


def handle_sigint(_signal, _frame, app: QApplication):
    app.exit(0)
    exit(0)


def main():
    app = QApplication(sys.argv)
    signal.signal(
        signal.SIGINT, lambda signal, frame: handle_sigint(signal, frame, app)
    )

    top_level = QStackedWidget()
    main_screen(top_level)
    top_level.show()

    sigint_timer = QTimer(top_level, interval=100)
    sigint_timer.timeout.connect(lambda: None)
    sigint_timer.start()

    app.exec()


def main_screen(stack: QStackedWidget):
    layout = QVBoxLayout()
    layout.setAlignment(Qt.AlignmentFlag.AlignCenter)

    title = QLabel("# Sovereign Chess", textFormat=Qt.TextFormat.MarkdownText)
    layout.addWidget(title)

    standard = QPushButton("Standard")
    standard.clicked.connect(lambda: play_screen(stack, VariantPreset("Standard")))
    layout.addWidget(standard)

    arena = QPushButton("Arena")
    arena.clicked.connect(lambda: play_screen(stack, VariantPreset("Arena")))
    layout.addWidget(arena)

    widget = QWidget()
    widget.setLayout(layout)

    index = stack.addWidget(widget)
    stack.setCurrentIndex(index)


def play_screen(stack: QStackedWidget, preset: VariantPreset):
    game = SessionAdapter.new_from_preset(preset)
    text = QLabel(str(game.game_view())[:], wordWrap=True)

    index = stack.addWidget(text)
    stack.setCurrentIndex(index)
