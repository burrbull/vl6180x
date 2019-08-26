

int VL6180x_GetCachedDWord(&mut self, uint16_t  index, uint32_t *pValue)
{
    int status;
    uint32_t Value;
    if (VL6180xDevDataGet(dev, CacheFilled) != 0 &&
        index >= VL6180x_FIRST_CACHED_INDEX  &&
        index <= (VL6180x_LAST_CACHED_INDEX - 3)) {
        uint8_t *pBytes = &VL6180xDevDataGet(dev, CachedRegs[index - VL6180x_FIRST_CACHED_INDEX]);
        Value = ((uint32_t)pBytes[0] << 24) |
                ((uint32_t)pBytes[1] << 16) |
                ((uint32_t)pBytes[2] << 8) |
                (uint32_t)pBytes[3];
        *pValue = Value;
        status = 0;
    } else {
        status =  self.read(index, pValue).bits();
    }
    return status;
}

int VL6180x_GetCachedWord(&mut self, uint16_t  index, uint16_t *pValue)
{
    int status;
    uint32_t Value;
    if (VL6180xDevDataGet(dev, CacheFilled) != 0 &&
        index >= VL6180x_FIRST_CACHED_INDEX  &&
        index <= (VL6180x_LAST_CACHED_INDEX - 1)) {
        uint8_t *pBytes = &VL6180xDevDataGet(dev, CachedRegs[index - VL6180x_FIRST_CACHED_INDEX]);
        Value = ((uint32_t)pBytes[0] << 8) | (uint32_t)pBytes[1];
        *pValue = Value;
        status = 0;
    } else {
        status =  self.read(index, pValue).bits();
    }
    return status;
}

int VL6180x_GetCachedByte(&mut self, uint16_t  index, uint8_t *pValue)
{
    int status;
    uint8_t Value;
    if (VL6180xDevDataGet(dev, CacheFilled) != 0 &&
        index >= VL6180x_FIRST_CACHED_INDEX &&
        index <= VL6180x_LAST_CACHED_INDEX) {
        Value = VL6180xDevDataGet(dev, CachedRegs[index - VL6180x_FIRST_CACHED_INDEX]);
        *pValue = Value;
        status = 0;
    } else {
        status =  self.read(index, pValue).bits();
    }
    return status;
}


int _CachedRegs_Fetch(&mut self)
{
    int status;
    uint8_t *Buffer;
    if (VL6180xDevDataGet(dev, CacheFilled) == 0) {
        self.dev.CacheFilled = 1;
        Buffer = &VL6180xDevDataGet(dev, CachedRegs[0]);
        status = VL6180x_RdMulti(dev, VL6180x_FIRST_CACHED_INDEX, Buffer, VL6180x_CACHED_REG_CNT);
    } else {
        status = 0 ;
    }
    return status;
}

void _CachedRegs_Flush(&mut self)
{
    self.dev.CacheFilled = 0;
}

#else
#   define _CachedRegs_Fetch(...) 0
#   define _CachedRegs_Flush(...) (void)0
#   define _Fetch_CachedRegs(...) 0
#   define VL6180x_GetCachedByte(dev, index, pValue) self.read(index, pValue).bits()
#   define VL6180x_GetCachedWord(dev, index, pValue) self.read(index, pValue).bits()
#   define VL6180x_GetCachedDWord(dev, index, pValue) self.read(index, pValue).bits()
