from pydantic import (
    BaseModel,
    ConfigDict,
    Field,
    SerializerFunctionWrapHandler,
)
from pydantic.alias_generators import to_pascal
from typing import TypeAlias, Annotated, Any


class EnumVariant(BaseModel):
    model_config = ConfigDict(
        alias_generator=to_pascal, validate_by_alias=True, validate_by_name=True, serialize_by_alias=True
    )


def dict_validator(value: Any, key_model: type[BaseModel] | None = None) -> Any:
    dict_value = {}
    if isinstance(value, dict):
        return value
    try:
        for k, v in value:
            if key_model is not None:
                k = key_model.model_validate(k)
            dict_value[k] = v
    except:
        raise ValueError("Failed to extract dict")

    return dict_value


def dict_serializer(value: Any, handler: SerializerFunctionWrapHandler) -> list[tuple]:
    if isinstance(value, list):
        try:
            for _, _ in value:
                pass
            return handler(value)
        except Exception as e:
            raise ValueError("Passed list is not of tuple pair").with_traceback(
                e.__traceback__
            )

    if not isinstance(value, dict):
        raise ValueError("Expected a dict")

    return handler([(k, v) for k, v in value.items()])


UintField: TypeAlias = Annotated[int, Field(ge=0)]
