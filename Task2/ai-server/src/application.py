from datetime import datetime

import pyjwt

from . import dto, infostructure
from .domain import UserExercise, user_exercises


async def create_exercise() -> int:
    try:
        last_exercise_id = await infostructure.get_last_exercise()
        user_exercises[last_exercise_id[0]] = UserExercise()
        return last_exercise_id[0]
    except:
        raise ValueError("Exercise not exists")


async def update_exercise(exercise_id: int, update: dto.UpdateUserExercise):
    try:
        user_exercises[exercise_id].states.append(update.user_state)
        if update.add_count is not None:
            count = user_exercises[exercise_id].count or 0
            user_exercises[exercise_id].count = count + update.add_count
    except KeyError:
        raise ValueError("Exercise not exists")


async def create_predict(exercise_id: int) -> dto.AIResponse:
    duration = (datetime.now() - user_exercises[exercise_id].start_at).seconds
    predict = (10 - duration // 2) % 11
    return dto.AIResponse(will_continue=predict)


async def start_exercise(jwt_token: str):
    try:
        exercise_user_id, user_id = infostructure.get_iot_exercise(iot_id.id)
        exercise_user: UserExercise = infostructure.get_user_exercise(exercise_user_id)
        exercise_user.start_at = datetime.now()
        user_exercises[user_id] = exercise_user
    except:
        raise ValueError("Exercise not exists")
