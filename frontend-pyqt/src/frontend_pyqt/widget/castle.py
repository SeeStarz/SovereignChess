from PySide6.QtWidgets import QPushButton
from PySide6.QtGui import QIcon, QPixmap
from adapter_py.model.engine import PieceType, PieceSimple, FactionId
from frontend_pyqt.helper import piece_image


def get_castle_widget():
    piece = PieceSimple(
        faction=FactionId(root=0),
        piece_type=PieceType(root="Rook"),
    )
    image = piece_image(piece, FactionId(root=0))
    icon = QIcon(QPixmap.fromImage(image))

    button = QPushButton()
    button.setIcon(icon)

    return button
