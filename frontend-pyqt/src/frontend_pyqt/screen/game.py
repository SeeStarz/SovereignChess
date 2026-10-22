from PySide6.QtWidgets import (
    QVBoxLayout,
    QLabel,
    QWidget,
    QStackedWidget,
    QPushButton,
    QHBoxLayout,
)
from PySide6.QtCore import Qt
from adapter_py.api.session import SessionAdapter
from adapter_py.model.engine import VariantPreset
from frontend_pyqt.widget.board import BoardWidget
from frontend_pyqt.widget.promotion import get_promotion_widget
from frontend_pyqt.logic.game_controller import GameController
from frontend_pyqt.helper import pop_stack


def game_screen(stack: QStackedWidget, preset: VariantPreset) -> QWidget:
    layout = QHBoxLayout()
    layout.setAlignment(Qt.AlignmentFlag.AlignCenter)

    session = SessionAdapter.new_from_preset(preset)
    controller = GameController(session)

    board_widget = BoardWidget(session.game_view())
    board_widget.setFixedSize(520, 520)
    board_widget.on_gesture_made.connect(controller.do_board_gesture)
    controller.on_game_view_change.connect(board_widget.update_game_view)
    layout.addWidget(board_widget)

    menu = QVBoxLayout()
    menu.setAlignment(Qt.AlignmentFlag.AlignTop)
    layout.addLayout(menu)

    player_text = QLabel("## Player 1's Turn", textFormat=Qt.TextFormat.MarkdownText)
    menu.addWidget(player_text)

    swap = QPushButton("Swap")
    swap.clicked.connect(lambda: controller.do_swap(True))
    menu.addWidget(swap)
    stay = QPushButton("Stay")
    swap.clicked.connect(lambda: controller.do_swap(False))
    menu.addWidget(stay)

    promotion = get_promotion_widget()
    menu.addWidget(promotion)

    back = QPushButton("Back")
    menu.addWidget(back)

    widget = QWidget()
    widget.setLayout(layout)
    controller.setParent(widget)

    back.clicked.connect(lambda: pop_stack(stack))

    return widget
