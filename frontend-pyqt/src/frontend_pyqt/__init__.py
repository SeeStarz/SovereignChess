from PySide6.QtWidgets import (
    QApplication,
    QVBoxLayout,
    QLabel,
    QWidget,
    QStackedWidget,
    QPushButton,
)
from PySide6.QtCore import Qt, QTimer, QRect, QSize, QPoint, QSizeF, QPointF
from PySide6.QtGui import QPainter, QColor, QPaintEvent, QPen
from adapter_py.api.session import SessionAdapter
from adapter_py.model.engine import VariantPreset, BoardView, ColorStandard, FactionId
from adapter_py.model.adapter import GameView
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


def play_screen(stack: QStackedWidget, preset: VariantPreset) -> QWidget:
    layout = QVBoxLayout()
    layout.setAlignment(Qt.AlignmentFlag.AlignCenter)

    game = SessionAdapter.new_from_preset(preset)
    board_widget = BoardWidget(game.game_view())
    board_widget.setFixedSize(520, 520)
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

    def __init__(self, game_view: GameView, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.game_view = game_view

    def paintEvent(self, _event: QPaintEvent):
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
