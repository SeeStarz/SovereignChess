from PySide6.QtWidgets import QApplication, QStackedWidget
from PySide6.QtCore import QTimer
from frontend_pyside6.helper import push_stack
from frontend_pyside6 import screen
import sys
import signal


def handle_sigint(_signal, _frame, app: QApplication):
    app.exit(0)


def main():
    app = QApplication(sys.argv)
    app.setStyle("fusion")

    signal.signal(
        signal.SIGINT, lambda signal, frame: handle_sigint(signal, frame, app)
    )

    stack = QStackedWidget()
    push_stack(stack, screen.main_screen(stack))
    stack.show()

    # This is so that python is waken up every 100ms to process signals
    sigint_timer = QTimer(stack, interval=100)
    sigint_timer.timeout.connect(lambda: None)
    sigint_timer.start()

    app.exec()
