# This file is part of the Trezor project.
#
# Copyright (C) SatoshiLabs and contributors
#
# This library is free software: you can redistribute it and/or modify
# it under the terms of the GNU Lesser General Public License version 3
# as published by the Free Software Foundation.
#
# This library is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU Lesser General Public License for more details.
#
# You should have received a copy of the License along with this library.
# If not, see <https://www.gnu.org/licenses/lgpl-3.0.html>.

from hashlib import blake2s

import pytest

from trezorlib import firmware, messages, models
from trezorlib.debuglink import DebugSession as Session

PIN4 = "1234"

# size of FIRMWARE_MAXSIZE, see core/embed/models/<model>/model_<model>.h
FIRMWARE_LENGTHS = {
    models.T1B1: 7 * 128 * 1024 + 64 * 1024,
    models.T2T1: 13 * 128 * 1024,
    models.T2B1: 13 * 128 * 1024,
    models.T3T1: 208 * 8 * 1024,
    models.T3B1: 208 * 8 * 1024,
    models.T3W1: 417 * 8 * 1024,
}


def test_firmware_hash_emu(session: Session) -> None:
    if not session.test_ctx.is_emulator:
        pytest.skip("Only for emulator")

    data = b"\xff" * FIRMWARE_LENGTHS[session.model]

    expected_hash = blake2s(data).digest()
    hash = firmware.get_hash(session, None)
    assert hash == expected_hash

    challenge = b"Hello Trezor"
    expected_hash = blake2s(data, key=challenge).digest()
    hash = firmware.get_hash(session, challenge)
    assert hash == expected_hash


def test_firmware_hash_hw(session: Session) -> None:
    if session.test_ctx.is_emulator:
        pytest.skip("Only for hardware")

    # TODO get firmware image from outside the environment, check for actual result
    challenge = b"Hello Trezor"
    empty_data = b"\xff" * FIRMWARE_LENGTHS[session.model]
    empty_hash = blake2s(empty_data).digest()
    empty_hash_challenge = blake2s(empty_data, key=challenge).digest()

    hash = firmware.get_hash(session, None)
    assert hash != empty_hash

    hash2 = firmware.get_hash(session, challenge)
    assert hash != hash2
    assert hash2 != empty_hash_challenge


@pytest.mark.setup_client(pin=PIN4)
def test_firmware_hash_locked(session: Session) -> None:
    assert session.features.unlocked is False

    challenge = b"Hello Trezor"
    with session.test_ctx as client:
        # no PIN prompt is expected
        client.set_expected_responses([messages.FirmwareHash, messages.FirmwareHash])
        hash_locked = firmware.get_hash(session, None)
        hash_locked_challenge = firmware.get_hash(session, challenge)

    session.refresh_features()
    assert session.features.unlocked is False

    with session.test_ctx as client:
        client.use_pin_sequence([PIN4])
        session.ensure_unlocked()
    assert session.features.unlocked is True

    assert firmware.get_hash(session, None) == hash_locked
    assert firmware.get_hash(session, challenge) == hash_locked_challenge
