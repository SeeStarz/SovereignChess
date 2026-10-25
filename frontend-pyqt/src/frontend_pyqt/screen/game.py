from PySide6.QtWidgets import (
    QVBoxLayout,
    QLabel,
    QWidget,
    QStackedWidget,
    QPushButton,
    QHBoxLayout,
)
from PySide6.QtCore import Qt, QSize
from adapter_py.api.session import SessionAdapter
from adapter_py.model.engine import VariantPreset
from frontend_pyqt.widget import (
    BoardWidget,
    PromotionWidget,
    DefectionWidget,
    get_castle_widget,
)
from frontend_pyqt.logic.game_controller import GameController
from frontend_pyqt.helper import pop_stack


def game_screen(stack: QStackedWidget, preset: VariantPreset) -> QWidget:
    layout = QHBoxLayout()
    layout.setAlignment(Qt.AlignmentFlag.AlignCenter)

    session = SessionAdapter.new_from_preset(preset)
    controller = GameController(session)

    tile_size = 32
    board_size = QSize(
        tile_size * session.game_view().game_state.board_view.width,
        tile_size * session.game_view().game_state.board_view.height,
    )
    board_size += QSize(BoardWidget.MARGIN * 2, BoardWidget.MARGIN * 2)
    board_widget = BoardWidget(session.game_view())
    board_widget.setFixedSize(board_size)
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
    stay.clicked.connect(lambda: controller.do_swap(False))
    menu.addWidget(stay)

    promotion = PromotionWidget()
    promotion.on_promotion_selected.connect(controller.do_promotion)
    menu.addWidget(promotion)

    defection = DefectionWidget(session.game_view().game_state.factions)
    defection.on_defection_selected.connect(controller.do_defection)
    menu.addWidget(defection)

    castle = get_castle_widget()
    castle.clicked.connect(controller.do_castle)
    menu.addWidget(castle)

    back = QPushButton("Back")
    menu.addWidget(back)

    widget = QWidget()
    widget.setLayout(layout)
    controller.setParent(widget)

    back.clicked.connect(lambda: pop_stack(stack))

    return widget
