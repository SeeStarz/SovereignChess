from PySide6.QtWidgets import QWidget
from PySide6.QtCore import QRect, QSize, QPoint, QSizeF, QPointF, Signal, Slot
from PySide6.QtGui import QMouseEvent, QPainter, QColor, QPaintEvent, QPen
from adapter_py.model.engine import BoardView
from adapter_py.model.adapter import (
    GameView,
    Gesture,
    GestureBoardVariant,
    BoardGesture,
    BoardGestureClickVariant,
)
from adapter_py.model.engine import Coordinate, PieceSimple
from frontend_pyqt.helper import faction_to_color, piece_image


class BoardWidget(QWidget):
    game_view: GameView
    on_gesture_made: Signal = Signal(Gesture)

    def __init__(self, game_view: GameView, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.game_view = game_view

    @Slot(GameView)
    def update_game_view(self, game_view: GameView):
        self.game_view = game_view
        self.repaint()

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
        image = piece_image(PieceSimple.from_rich(piece), piece.owner)
        painter.drawImage(subrect, image)


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
