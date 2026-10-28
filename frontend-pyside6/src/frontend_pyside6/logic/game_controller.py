from PySide6.QtCore import QObject, Slot, Signal, QPoint
from adapter_py.api.session import SessionAdapter
from adapter_py.model.engine import PieceType, FactionId, PlayerId
from adapter_py.model.adapter import (
    Gesture,
    GestureBoardVariant,
    BoardGesture,
    GestureMenuClickVariant,
    GestureCancelVariant,
    MenuClick,
    MenuClickPromotionVariant,
    MenuClickDefectionVariant,
    SessionAction,
    SessionActionGestureVariant,
    SessionActionSwapVariant,
    GameView,
)
from frontend_pyside6.setting import SettingLoader


class GameController(QObject):
    session: SessionAdapter
    cached_view: GameView

    on_game_view_change: Signal = Signal(GameView)
    on_promotion_options: Signal = Signal(list)
    on_promotion_faction_and_owner: Signal = Signal(FactionId, FactionId)
    on_promotion_location: Signal = Signal(QPoint)
    on_swap_prompt_enabled: Signal = Signal(bool)
    on_defection_options: Signal = Signal(list)
    on_defection_deselect: Signal = Signal()
    on_player_turn: Signal = Signal(PlayerId)

    def __init__(self, session: SessionAdapter):
        super().__init__()
        self.session = session
        self.cached_view = self.session.game_view()

    @Slot()
    def do_cancel(self):
        self.do_action(
            SessionAction(
                root=SessionActionGestureVariant(
                    gesture=Gesture(root=GestureCancelVariant(root="Cancel"))
                )
            )
        )

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
        self.on_defection_options.emit(
            self.cached_view.intra_game_state.valid_defections
        )

        if self.cached_view.intra_game_state.promotion_at is not None:
            board_margin = SettingLoader.get().board_margin
            tile_size = SettingLoader.get().tile_size
            promotion_coordinate = self.cached_view.intra_game_state.promotion_at
            point = QPoint(
                tile_size * promotion_coordinate.col + board_margin,
                tile_size * promotion_coordinate.row + board_margin,
            )
            self.on_promotion_location.emit(point)

            assert (
                self.cached_view.intra_game_state.grabbed_piece is not None
            ), "Grabbed piece is None despite being on promotion"

            faction = self.cached_view.intra_game_state.grabbed_piece.faction
            owner = self.cached_view.intra_game_state.grabbed_piece.owner
            assert (
                owner is not None
            ), "Grabbed piece owner is None despite being on promotion"

            self.on_promotion_faction_and_owner.emit(faction, owner)

        if self.cached_view.intra_game_state.selected_defection is None:
            self.on_defection_deselect.emit()
