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
from adapter_py.model.adapter import Gesture, SessionAction, SessionActionGestureVariant
from frontend_pyqt.board import BoardWidget
import sys
import signal


def handle_sigint(_signal, _frame, app: QApplication):
    app.exit(0)


def main():
    app = QApplication(sys.argv)
    signal.signal(
        signal.SIGINT, lambda signal, frame: handle_sigint(signal, frame, app)
    )

    stack = QStackedWidget()
    push_stack(stack, main_screen(stack))
    stack.show()

    # This is so that python is waken up every 100ms to process signals
    sigint_timer = QTimer(stack, interval=100)
    sigint_timer.timeout.connect(lambda: None)
    sigint_timer.start()

    app.exec()


def main_screen(stack: QStackedWidget):
    layout = QVBoxLayout()
    layout.setAlignment(Qt.AlignmentFlag.AlignCenter)

    title = QLabel("# Sovereign Chess", textFormat=Qt.TextFormat.MarkdownText)
    layout.addWidget(title)

    standard = QPushButton("Standard")
    standard.clicked.connect(
        lambda: push_stack(stack, play_screen(stack, VariantPreset("Standard")))
    )
    layout.addWidget(standard)

    arena = QPushButton("Arena")
    arena.clicked.connect(
        lambda: push_stack(stack, play_screen(stack, VariantPreset("Arena")))
    )
    layout.addWidget(arena)

    widget = QWidget()
    widget.setLayout(layout)

    return widget


def play_screen(stack: QStackedWidget, preset: VariantPreset) -> QWidget:
    layout = QVBoxLayout()
    layout.setAlignment(Qt.AlignmentFlag.AlignCenter)

    game = SessionAdapter.new_from_preset(preset)
    board_widget = BoardWidget(game.game_view())
    board_widget.setFixedSize(520, 520)
    board_widget.on_gesture_made.connect(
        lambda gesture: mock(game, board_widget, gesture)
    )
    layout.addWidget(board_widget)

    back = QPushButton("Back")
    layout.addWidget(back)

    widget = QWidget()
    widget.setLayout(layout)

    back.clicked.connect(lambda: pop_stack(stack))

    return widget


def push_stack(stack: QStackedWidget, widget: QWidget):
    stack.addWidget(widget)
    stack.setCurrentWidget(widget)


def pop_stack(stack: QStackedWidget):
    widget = stack.currentWidget()
    assert widget is not None
    stack.removeWidget(widget)
    widget.deleteLater()
    stack.setCurrentIndex(stack.count() - 1)


def mock(session: SessionAdapter, board_widget: BoardWidget, gesture: Gesture):
    session.do_action(SessionAction(root=SessionActionGestureVariant(gesture=gesture)))
    board_widget.game_view = session.game_view()
    board_widget.repaint()
