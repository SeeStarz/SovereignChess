from PySide6.QtGui import QColor
from dataclasses import dataclass
from adapter_py.model.engine import ColorStandard


@dataclass(frozen=True)
class Setting:
    tile_size: int
    faction_color_map: dict[ColorStandard, QColor]
    promotion_line_color: QColor
    move_marker_color: QColor
    light_tile_color: QColor
    dark_tile_color: QColor
    board_disable_color: QColor
    online_indicator_color: QColor
    close_spacing: int


class SettingLoader:
    _value: Setting | None = None

    @classmethod
    def get(cls) -> Setting:
        if cls._value is None:
            cls._value = Setting(
                tile_size=32,
                faction_color_map={
                    k: QColor.fromRgb(*v)
                    for k, v in {
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
                    }.items()
                },
                promotion_line_color=QColor.fromRgb(110, 38, 14),
                move_marker_color=QColor.fromRgb(0, 0, 255),
                light_tile_color=QColor.fromRgb(255, 248, 220),
                dark_tile_color=QColor.fromRgb(222, 184, 135),
                board_disable_color=QColor.fromRgb(255, 255, 255, 127),
                online_indicator_color=QColor.fromRgb(0, 255, 0),
                close_spacing=2,
            )
        return cls._value
