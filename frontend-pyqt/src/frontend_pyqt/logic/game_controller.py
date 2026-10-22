from PySide6.QtCore import QObject, Slot, Signal
from adapter_py.api.session import SessionAdapter
from adapter_py.model.engine import PieceType, FactionId
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
        self.cached_view = self.session.game_view()
        self.on_game_view_change.emit(self.cached_view)
