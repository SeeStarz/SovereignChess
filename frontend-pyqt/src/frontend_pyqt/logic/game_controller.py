from PySide6.QtCore import QObject, Slot, Signal
from adapter_py.api.session import SessionAdapter
from adapter_py.model.engine import PieceType, FactionId, PlayerId
from adapter_py.model.adapter import (
    Gesture,
    GestureBoardVariant,
    BoardGesture,
    GestureMenuClickVariant,
    MenuClick,
    MenuClickPromotionVariant,
    MenuClickCastleVariant,
    MenuClickDefectionVariant,
    SessionAction,
    SessionActionGestureVariant,
    SessionActionSwapVariant,
    GameView,
)


class GameController(QObject):
    session: SessionAdapter
    cached_view: GameView

    on_game_view_change: Signal = Signal(GameView)
    on_promotion_options: Signal = Signal(list)
    on_swap_prompt_enabled: Signal = Signal(bool)
    on_player_turn: Signal = Signal(PlayerId)

    def __init__(self, session: SessionAdapter):
        super().__init__()
        self.session = session
        self.cached_view = self.session.game_view()

    @Slot(Gesture)
    def do_board_gesture(self, gesture: BoardGesture):
        self.do_action(
            SessionAction(
                root=SessionActionGestureVariant(
                    gesture=Gesture(root=GestureBoardVariant(board=gesture))
                )
            )
        )

    @Slot(bool)
    def do_swap(self, swap: bool):
        self.do_action(SessionAction(root=SessionActionSwapVariant(swap=swap)))

    @Slot(PieceType)
    def do_promotion(self, piece_type: PieceType):
        self.do_menu_click(
            MenuClick(root=MenuClickPromotionVariant(promotion=piece_type))
        )

    @Slot(FactionId)
    def do_defection(self, faction: FactionId):
        self.do_menu_click(MenuClick(root=MenuClickDefectionVariant(defection=faction)))

    @Slot()
    def do_castle(self):
        self.do_menu_click(MenuClick(root=MenuClickCastleVariant(root="Castle")))

    def do_menu_click(self, menu_click: MenuClick):
        self.do_action(
            SessionAction(
                root=SessionActionGestureVariant(
                    gesture=Gesture(root=GestureMenuClickVariant(menu_click=menu_click))
                )
            )
        )

    def do_action(self, action: SessionAction):
        self.session.do_action(action)
        self.emit_signals()

    def emit_signals(self):
        self.cached_view = self.session.game_view()
        self.on_game_view_change.emit(self.cached_view)
        self.on_promotion_options.emit(
            self.cached_view.intra_game_state.promotion_options
        )
        self.on_swap_prompt_enabled.emit(self.cached_view.game_state.waiting_swap)
        self.on_player_turn.emit(self.cached_view.game_state.turn_to_play)
