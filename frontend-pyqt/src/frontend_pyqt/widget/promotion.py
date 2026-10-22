from PySide6.QtWidgets import QWidget, QHBoxLayout, QPushButton
from PySide6.QtGui import QIcon, QPixmap
from adapter_py.model.engine import PieceType, PieceSimple, FactionId
from frontend_pyqt.helper import piece_image


def get_promotion_widget() -> QWidget:
    layout = QHBoxLayout()

    for piece_type in ("King", "Queen", "Rook", "Bishop", "Knight"):
        piece = PieceSimple(
            faction=FactionId(root=0),
            piece_type=PieceType(root=piece_type),
        )
        image = piece_image(piece, FactionId(root=0))
        icon = QIcon(QPixmap.fromImage(image))

        button = QPushButton()
        button.setIcon(icon)
        layout.addWidget(button)

    widget = QWidget()
    widget.setLayout(layout)

    return widget
