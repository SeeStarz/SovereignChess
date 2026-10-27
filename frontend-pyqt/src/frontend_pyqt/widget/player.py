from PySide6.QtWidgets import (
    QWidget,
    QVBoxLayout,
    QHBoxLayout,
    QLabel,
)
from PySide6.QtGui import QPainter, QColor, QPen, QColorConstants
from PySide6.QtCore import Slot, QSize, Qt, QRect, QPoint
from adapter_py.model.engine import PlayerId
from adapter_py.model.adapter import GameView
from frontend_pyqt.helper import faction_to_color
from frontend_pyqt.setting import SettingLoader


class SquareColored(QWidget):
    color: QColor

    def __init__(self, color: QColor):
        super().__init__()
        self.color = color

    def paintEvent(self, event):
        super().paintEvent(event)

        painter = QPainter(self)
        painter.fillRect(QRect(QPoint(0, 0), self.size()), QColor.fromRgb(0, 0, 0))
        painter.fillRect(QRect(QPoint(2, 2), self.size() - QSize(4, 4)), self.color)


class CircleColored(QWidget):
    color: QColor

    def __init__(self, color: QColor):
        super().__init__()
        self.color = color

    def paintEvent(self, event):
        super().paintEvent(event)

        painter = QPainter(self)
        painter.setBrush(SettingLoader.get().online_indicator_color)
        painter.setPen(QPen(QColorConstants.Black, 2))
        painter.drawEllipse(QRect(QPoint(2, 2), self.size() - QSize(4, 4)))
        # painter.fillRect(QRect(QPoint(2, 2), self.size() - QSize(4, 4)), self.color)


class PlayerWidget(QWidget):
    game_view: GameView
    widget_map: dict[PlayerId, QWidget]
    square_map: dict[PlayerId, SquareColored]

    def __init__(self, game_view: GameView):
        super().__init__()

        self.game_view = game_view

        vertical = QVBoxLayout()
        vertical.setContentsMargins(0, 0, 0, 0)

        self.widget_map = {}
        self.square_map = {}
        for i in range(game_view.game_state.player_count):
            horizontal = QHBoxLayout()
            horizontal.setAlignment(Qt.AlignmentFlag.AlignLeft)
            horizontal.setContentsMargins(0, 0, 0, 0)

            player_id = PlayerId(root=i)
            faction = game_view.game_state.player_main_factions[player_id]
            color = faction_to_color(faction)

            square = SquareColored(color)
            square.setFixedSize(16, 16)
            horizontal.addWidget(square)

            label = QLabel("Player " + str(i))
            horizontal.addWidget(label)

            circle = CircleColored(QColor.fromRgb(255, 0, 0))
            circle.setFixedSize(16, 16)
            horizontal.addWidget(circle)

            widget = QWidget()
            widget.setLayout(horizontal)

            if player_id != game_view.game_state.turn_to_play:
                widget.setDisabled(True)
            self.widget_map[player_id] = widget
            self.square_map[player_id] = square

            vertical.addWidget(widget)

        self.setLayout(vertical)

    @Slot(GameView)
    def update_game_view(self, game_view: GameView):
        self.game_view = game_view

        for player_id, widget in self.widget_map.items():
            widget.setEnabled(player_id == self.game_view.game_state.turn_to_play)

        for player_id, square in self.square_map.items():
            faction = self.game_view.game_state.player_main_factions[player_id]
            square.color = faction_to_color(faction)
            square.repaint()
