from PySide6.QtWidgets import (
    QLayout,
    QVBoxLayout,
    QLabel,
    QWidget,
    QStackedWidget,
    QPushButton,
    QHBoxLayout,
    QStackedLayout,
    QGroupBox,
    QSizePolicy,
)
from PySide6.QtCore import Qt, QSize
from adapter_py.api.session import SessionAdapter, GameView
from adapter_py.model.engine import VariantPreset, FactionId
from frontend_pyqt.widget import (
    BoardWidget,
    PromotionWidget,
    DefectionWidget,
    get_castle_widget,
)
from frontend_pyqt.logic.game_controller import GameController
from frontend_pyqt.helper import pop_stack
from frontend_pyqt.setting import SettingLoader


def game_screen(stack: QStackedWidget, preset: VariantPreset) -> QWidget:
    horizontal = QHBoxLayout()
    horizontal.setAlignment(Qt.AlignmentFlag.AlignCenter)

    session = SessionAdapter.new_from_preset(preset)
    controller = GameController(session)

    board_widget = board_group(controller, session.game_view())
    horizontal.addWidget(board_widget)

    menu = menu_group(stack, controller, session.game_view().game_state.factions)
    horizontal.addWidget(menu)

    widget = QWidget()
    widget.setLayout(horizontal)
    controller.setParent(widget)

    return widget


def menu_group(
    stack: QStackedWidget, controller: GameController, faction_list: list[FactionId]
) -> QWidget:
    vertical = QVBoxLayout()
    vertical.setAlignment(Qt.AlignmentFlag.AlignTop)

    player_text = QLabel("## Player 1's Turn", textFormat=Qt.TextFormat.MarkdownText)
    vertical.addWidget(player_text)

    defection = defection_group(controller, faction_list)
    vertical.addWidget(defection)

    castle = get_castle_widget()
    castle.clicked.connect(controller.do_castle)
    vertical.addWidget(castle)

    back = QPushButton("Back")
    back.clicked.connect(lambda: pop_stack(stack))
    # back.setFixedWidth(400)
    vertical.addWidget(back)

    panel = QWidget()
    panel.setSizePolicy(QSizePolicy.Policy.Maximum, QSizePolicy.Policy.Preferred)
    panel.setLayout(vertical)
    print(panel.sizeHint(), panel.minimumSizeHint())

    return panel


def board_group(controller: GameController, game_view: GameView) -> QWidget:
    board_size = QSize(
        SettingLoader.get().tile_size * game_view.game_state.board_view.width,
        SettingLoader.get().tile_size * game_view.game_state.board_view.height,
    )
    board_size += QSize(BoardWidget.MARGIN * 2, BoardWidget.MARGIN * 2)
    board_widget = BoardWidget(game_view)
    board_widget.setFixedSize(board_size)
    board_widget.on_gesture_made.connect(controller.do_board_gesture)
    controller.on_game_view_change.connect(board_widget.update_game_view)

    stacked = QStackedLayout(stackingMode=QStackedLayout.StackingMode.StackAll)

    swap_widget = QWidget()
    swap_widget.setLayout(swap_group(controller))
    stacked.addWidget(swap_widget)

    promotion = PromotionWidget()
    promotion.on_promotion_selected.connect(controller.do_promotion)
    stacked.addWidget(promotion)

    board_widget.setLayout(stacked)

    return board_widget


def swap_group(controller: GameController) -> QLayout:
    vertical = QVBoxLayout()
    vertical.setAlignment(Qt.AlignmentFlag.AlignCenter)

    swap = QPushButton("Swap")
    swap.clicked.connect(lambda: controller.do_swap(True))
    vertical.addWidget(swap)

    stay = QPushButton("Stay")
    stay.clicked.connect(lambda: controller.do_swap(False))
    vertical.addWidget(stay)

    return vertical


def defection_group(
    controller: GameController, faction_list: list[FactionId]
) -> QWidget:
    box = QGroupBox()
    box.setTitle("Defection Controls")
    box.setSizePolicy(QSizePolicy.Policy.Maximum, QSizePolicy.Policy.Maximum)
    layout = QHBoxLayout()
    layout.setContentsMargins(0, 0, 0, 0)

    defection = DefectionWidget(faction_list)
    defection.on_defection_selected.connect(controller.do_defection)
    layout.addWidget(defection)

    box.setLayout(layout)
    return box
