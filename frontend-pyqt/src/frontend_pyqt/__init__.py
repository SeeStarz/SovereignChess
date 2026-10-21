from PySide6.QtWidgets import (
    QApplication,
    QVBoxLayout,
    QLabel,
    QWidget,
    QStackedWidget,
    QPushButton,
)
from PySide6.QtCore import Qt, QTimer, QRect, QSize, QPoint, QSizeF, QPointF, Signal
from PySide6.QtGui import QMouseEvent, QPainter, QColor, QPaintEvent, QPen, QImage
from adapter_py.api.session import SessionAdapter
from adapter_py.model.engine import (
    VariantPreset,
    BoardView,
    ColorStandard,
    FactionId,
    PieceRich,
)
from adapter_py.model.adapter import (
    GameView,
    Gesture,
    GestureBoardVariant,
    BoardGesture,
    BoardGestureClickVariant,
    SessionAction,
    SessionActionGestureVariant,
)
from adapter_py.model.engine import Coordinate
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


def mock(session: SessionAdapter, board_widget: BoardWidget, gesture: Gesture):
    session.do_action(SessionAction(root=SessionActionGestureVariant(gesture=gesture)))
    board_widget.game_view = session.game_view()
    board_widget.repaint()


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


class BoardWidget(QWidget):
    game_view: GameView
    on_gesture_made: Signal = Signal(Gesture)

    def __init__(self, game_view: GameView, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.game_view = game_view

    def paintEvent(self, event: QPaintEvent):
        super().paintEvent(event)

        width = self.width() - 8
        height = self.height() - 8
        size = QSize(width, height)
        rect = QRect(QPoint(4, 4), size)
        with QPainter(self) as painter:
            paint_board(
                self.game_view.game_state.board_view,
                painter,
                rect,
            )
            paint_pieces(self.game_view.game_state.board_view, painter, rect)

    def mouseReleaseEvent(self, event: QMouseEvent):
        super().mouseReleaseEvent(event)

        width = self.width() - 8
        height = self.height() - 8
        tile_width = width / self.game_view.game_state.board_view.width
        tile_height = height / self.game_view.game_state.board_view.height

        position = event.position()
        position -= QPointF(4.0, 4.0)

        tile_position = Coordinate(
            col=int(position.x() // tile_width), row=int(position.y() // tile_height)
        )

        click_gesture = BoardGestureClickVariant(click=tile_position)
        gesture = Gesture(
            root=GestureBoardVariant(board=BoardGesture(root=click_gesture))
        )

        self.on_gesture_made.emit(gesture)


def paint_board(board_view: BoardView, painter: QPainter, rect: QRect):
    old_pen = painter.pen()

    painter.setPen(QPen(QColor.fromRgb(0, 0, 0), 2))

    paint_tiles(board_view, painter, rect)
    paint_tile_lines(board_view, painter, rect)
    paint_promotion_lines(
        board_view, painter, rect, QPen(QColor.fromRgb(110, 38, 14), 4)
    )
    paint_promotion_area(board_view, painter, rect, QPen(QColor.fromRgb(0, 0, 0), 4))
    paint_border(painter, rect, QPen(QColor.fromRgb(0, 0, 0), 4))

    painter.setPen(old_pen)


def paint_tiles(board_view: BoardView, painter: QPainter, rect: QRect):
    for row in range(board_view.height):
        for col in range(board_view.width):
            subrect = calculate_subrect(
                board_view.height, board_view.width, row, col, rect
            )
            special_tile = board_view.rows[row].root[col].special
            if special_tile is not None:
                painter.fillRect(subrect, faction_to_color(special_tile.faction))
            elif (row + col) % 2 == 0:
                painter.fillRect(subrect, QColor.fromRgb(255, 248, 220))
            else:
                painter.fillRect(subrect, QColor.fromRgb(222, 184, 135))


def paint_tile_lines(board_view: BoardView, painter: QPainter, rect: QRect):
    size = calculate_subrect_size(board_view.height, board_view.width, rect)
    for row in range(board_view.height + 1):
        painter.drawLine(
            calculate_subrect_pos(size, row, 0, rect).toPoint(),
            calculate_subrect_pos(size, row, board_view.width, rect).toPoint(),
        )
    for col in range(board_view.width + 1):
        painter.drawLine(
            calculate_subrect_pos(size, 0, col, rect).toPoint(),
            calculate_subrect_pos(size, board_view.height, col, rect).toPoint(),
        )


def paint_promotion_area(
    board_view: BoardView, painter: QPainter, rect: QRect, pen: QPen
):
    old_pen = painter.pen()
    size = calculate_subrect_size(board_view.height, board_view.width, rect)
    painter.setPen(pen)

    promotion_area_bottom_right = calculate_subrect_pos(
        size,
        board_view.promotion_area.right + 1,
        board_view.promotion_area.bottom + 1,
        rect,
    )
    promotion_area_top_left = calculate_subrect_pos(
        size, board_view.promotion_area.left, board_view.promotion_area.top, rect
    )
    promotion_area_rect = QRect(
        promotion_area_top_left.toPoint(), promotion_area_bottom_right.toPoint()
    )
    painter.drawRect(promotion_area_rect)

    painter.setPen(old_pen)


def paint_promotion_lines(
    board_view: BoardView, painter: QPainter, rect: QRect, pen: QPen
):
    old_pen = painter.pen()
    size = calculate_subrect_size(board_view.height, board_view.width, rect)
    painter.setPen(pen)

    middle_row = (
        board_view.promotion_area.top + board_view.promotion_area.bottom + 1
    ) // 2
    middle_column = (
        board_view.promotion_area.left + board_view.promotion_area.right + 1
    ) // 2

    painter.drawLine(
        calculate_subrect_pos(size, middle_row, 0, rect).toPoint(),
        calculate_subrect_pos(
            size, middle_row, board_view.promotion_area.left, rect
        ).toPoint(),
    )
    painter.drawLine(
        calculate_subrect_pos(
            size, middle_row, board_view.promotion_area.right + 1, rect
        ).toPoint(),
        calculate_subrect_pos(size, middle_row, board_view.width, rect).toPoint(),
    )
    painter.drawLine(
        calculate_subrect_pos(size, 0, middle_column, rect).toPoint(),
        calculate_subrect_pos(
            size, board_view.promotion_area.top, middle_column, rect
        ).toPoint(),
    )
    painter.drawLine(
        calculate_subrect_pos(
            size, board_view.promotion_area.bottom + 1, middle_column, rect
        ).toPoint(),
        calculate_subrect_pos(size, board_view.height, middle_column, rect).toPoint(),
    )

    painter.setPen(old_pen)


def paint_border(painter: QPainter, rect: QRect, pen: QPen):
    old_pen = painter.pen()
    painter.setPen(pen)

    painter.drawRect(rect)

    painter.setPen(old_pen)


def paint_pieces(board_view: BoardView, painter: QPainter, rect: QRect):
    for piece in board_view.pieces:
        subrect = calculate_subrect(
            board_view.height,
            board_view.width,
            piece.coordinate.row,
            piece.coordinate.col,
            rect,
        )
        image = piece_image(piece)
        painter.drawImage(subrect, image)


def piece_image(piece: PieceRich) -> QImage:
    top_asset_path = "assets/img/{}.png".format(piece.piece_type.root.lower())
    base_asset_path = "assets/img/{}_base.png".format(piece.piece_type.root.lower())
    neutral_base_asset_path = "assets/img/{}_neutral.png".format(
        piece.piece_type.root.lower()
    )

    image = QImage(top_asset_path)
    top_color = faction_to_color(piece.faction)
    image = tint_image(image, top_color)

    with QPainter(image) as painter:
        if piece.owner is None:
            base_image = QImage(neutral_base_asset_path)
            painter.drawImage(image.rect(), base_image)
        else:
            base_image = QImage(base_asset_path)
            base_color = faction_to_color(piece.owner)
            base_image = tint_image(base_image, base_color)
            painter.drawImage(image.rect(), base_image)

    return image


def tint_image(image: QImage, color: QColor) -> QImage:
    image = image.convertToFormat(QImage.Format.Format_ARGB32_Premultiplied)
    mask = image.convertToFormat(QImage.Format.Format_Alpha8)

    with QPainter(image) as painter:
        painter.setCompositionMode(QPainter.CompositionMode.CompositionMode_Multiply)
        painter.fillRect(image.rect(), color)
        painter.setCompositionMode(
            QPainter.CompositionMode.CompositionMode_DestinationIn
        )
        painter.drawImage(image.rect(), mask)

    return image


def faction_to_color(faction_id: FactionId) -> QColor:
    DEFAULT = {
        ColorStandard.WHITE: (255, 255, 255),
        ColorStandard.PINK: (223, 159, 223),
        ColorStandard.SLATE: (127, 127, 127),
        ColorStandard.RED: (191, 31, 31),
        ColorStandard.ORANGE: (223, 95, 31),
        ColorStandard.YELLOW: (255, 255, 31),
        ColorStandard.GREEN: (31, 159, 31),
        ColorStandard.CYAN: (31, 191, 191),
        ColorStandard.NAVY: (31, 31, 191),
        ColorStandard.ASH: (207, 207, 207),
        ColorStandard.VIOLET: (159, 0, 233),
        ColorStandard.BLACK: (63, 63, 63),
    }

    color = DEFAULT.get(ColorStandard(faction_id.root))
    if color is None:
        print("Faction id", faction_id, "does not have a color")
        return QColor.fromRgb(255, 0, 0)

    return QColor.fromRgb(*color)


def calculate_subrect(
    row_count: int, column_count: int, row: int, col: int, rect: QRect
) -> QRect:
    size = calculate_subrect_size(row_count, column_count, rect)
    position = calculate_subrect_pos(size, row, col, rect)
    return QRect(position.toPoint(), size.toSize())


def calculate_subrect_pos(size: QSizeF, row: int, col: int, rect: QRect) -> QPointF:
    return QPointF(
        rect.left() + size.width() * col,
        rect.top() + size.height() * row,
    )


def calculate_subrect_size(row_count: int, column_count: int, rect: QRect):
    return QSizeF(rect.width() / column_count, rect.height() / row_count)
