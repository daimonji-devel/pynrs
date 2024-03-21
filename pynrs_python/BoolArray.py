class BoolArray:
    """
    Array of bool values which opimizes storage space by storing the bool values inside the bits of
    a Python bytearray.
    """
    _pos_masks = tuple((2**e for e in range(8)))
    _neg_masks = tuple((255 - value for value in _pos_masks))

    def __init__(self, init_len: int = 0, init_value: bool = False):
        init_byte = 0
        if init_value:
            init_byte = 255
        if init_len < 0:
            init_len = 0
        byte_len, bit_len = divmod(init_len, 8)
        if bit_len > 0:
            byte_len += 1
        self._bytes = bytearray((init_byte for _ in range(byte_len)))
        self._len = init_len

    def __len__(self):
        return self._len

    def __iter__(self):
        byte_len, bit_len = divmod(self._len, 8)
        for byte in self._bytes[:byte_len]:
            for mask in self._pos_masks:
                yield byte & mask == mask
        for mask in self._pos_masks[:bit_len]:
            yield self._bytes[-1] & mask == mask

    def __setitem__(self, index, value):        
        byte_index, bit_index = self._split_index(index)
        if value:
            self._bytes[byte_index] |= self._pos_masks[bit_index]
        else:
            self._bytes[byte_index] &= self._neg_masks[bit_index]

    def __getitem__(self, index):
        byte_index, bit_index = self._split_index(index)
        mask = self._pos_masks[bit_index]
        return self._bytes[byte_index] & mask == mask

    def append(self, value: bool):
        byte_index, bit_index = divmod(self._len, 8)
        if bit_index == 0:
            self._bytes.append(0)
        if value:
            self._bytes[byte_index] |= self._pos_masks[bit_index]
        else:
            self._bytes[byte_index] &= self._neg_masks[bit_index]
        self._len += 1

    def extend(self, values):
        for value in values:
            self.append(value)

    def _split_index(self, index: int) -> int:
        if index < 0:
            index = self._len + index
        if index >= self._len:
            raise IndexError("BoolArray index out of range")
        return divmod(index, 8)


if __name__ == '__main__':
    b = BoolArray()
    b.extend([True, False, True, True, False])
    for index in [0, 2, 3, -2, -3, -5]:
        assert b[index]
    for index in [1, 4, -1, -4]:
        assert not b[index]
    assert tuple(b) == (True, False, True, True, False), tuple(b)
    #
    b[4] = False
    assert not b[4]
    b[4] = True
    assert b[4]
    b[4] = True
    assert b[4]
    b[4] = False
    assert not b[4]

    for size in [0, 1, 8, 9, 24, 25]:
        assert tuple(BoolArray(size, True)) == (True,) * size
