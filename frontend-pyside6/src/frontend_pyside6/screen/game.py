from PySide6.QtWidgets import (
    QLayout,
    QVBoxLayout,
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
from frontend_pyside6.widget import (
    BoardWidget,
    PromotionWidget,
    DefectionWidget,
    PlayerWidget,
)
from frontend_pyside6.logic.game_controller import GameController
from frontend_pyside6.helper import pop_stack
from frontend_pyside6.setting import SettingLoader


def game_screen(stack: QStackedWidget, preset: VariantPreset) -> QWidget:
    horizontal = QHBoxLayout()
    horizontal.setAlignment(Qt.AlignmentFlag.AlignCenter)

    session = SessionAdapter.new_from_preset(preset)
    controller = GameController(session)

    board_widget = board_group(controller, session.game_view())
    horizontal.addWidget(board_widget)

    menu = menu_group(stack, controller, session.game_view())
    horizontal.addWidget(menu)

    widget = QWidget()
    widget.setLayout(horizontal)
    controller.setParent(widget)
    controller.emit_signals()

    return widget


def menu_group(
    stack: QStackedWidget, controller: GameController, game_view: GameView
) -> QWidget:
    vertical = QVBoxLayout()
    vertical.setAlignment(Qt.AlignmentFlag.AlignTop)

    player = PlayerWidget(game_view)
    controller.on_game_view_change.connect(player.update_game_view)
    vertical.addWidget(player)

    defection = defection_group(controller, game_view.game_state.factions)
    vertical.addWidget(defection)

    back = QPushButton("Back")
    back.clicked.connect(lambda: pop_stack(stack))
    vertical.addWidget(back)

    panel = QWidget()
    panel.setSizePolicy(QSizePolicy.Policy.Maximum, QSizePolicy.Policy.Preferred)
    panel.setLayout(vertical)

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
    controller.on_swap_prompt_enabled.connect(swap_widget.setVisible)
    stacked.addWidget(swap_widget)

    promotion = PromotionWidget()
    promotion.on_promotion_selected.connect(controller.do_promotion)
    controller.on_promotion_options.connect(promotion.update_active_promotions)
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
