from PySide6.QtCore import QObject, Slot, Signal
from adapter_py.api.session import SessionAdapter
from adapter_py.model.adapter import (
    Gesture,
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
    def do_board_gesture(self, gesture: Gesture):
        self.do_action(SessionAction(root=SessionActionGestureVariant(gesture=gesture)))

    @Slot(bool)
    def do_swap(self, swap: bool):
        self.do_action(SessionAction(root=SessionActionSwapVariant(swap=swap)))

    def do_action(self, action: SessionAction):
        self.session.do_action(action)
        self.cached_view = self.session.game_view()
        self.on_game_view_change.emit(self.cached_view)

        print(self.cached_view.game_state.board_view.pieces)
