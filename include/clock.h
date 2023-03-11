#pragma once

/* Generated with cbindgen:0.24.3 */

#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

#if defined(__APPLE__)
#include "TargetConditionals.h"
#endif


#if defined(TARGET_OS_OSX)
/**
 * A buffer that stores `LapTime` values for a stopwatch.
 */
typedef struct LapTimeBuffer {
  const void *data;
  size_t len;
} LapTimeBuffer;
#endif

#if defined(TARGET_OS_OSX)
/**
 * # Safety
 *
 * This function dereferences a raw pointer
 */
const char *stopwatch_formatTime(void *duration);
#endif

#if defined(TARGET_OS_OSX)
/**
 * # Safety
 *
 * This function dereferences a raw pointer
 */
void stopwatch_freeDuration(void *duration);
#endif

#if defined(TARGET_OS_OSX)
/**
 * # Safety
 *
 * This function dereferences a raw pointer
 */
void stopwatch_freeLapTime(void *laptime);
#endif

#if defined(TARGET_OS_OSX)
/**
 * # Safety
 *
 * This function dereferences a raw pointer
 */
void *stopwatch_laptime_getIndex(struct LapTimeBuffer buf, size_t index);
#endif

#if defined(TARGET_OS_OSX)
/**
 * # Safety
 *
 * This function dereferences a raw pointer
 */
uint32_t stopwatch_laptime_lapNumber(void *laptime);
#endif

#if defined(TARGET_OS_OSX)
/**
 * # Safety
 *
 * This function dereferences a raw pointer
 */
void *stopwatch_laptime_splitTime(void *laptime);
#endif

#if defined(TARGET_OS_OSX)
/**
 * # Safety
 *
 * This function dereferences a raw pointer
 */
void *stopwatch_laptime_totalTime(void *laptime);
#endif

#if defined(TARGET_OS_OSX)
/**
 * # Safety
 *
 * This function dereferences a raw pointer
 */
void stopwatch_stopwatch_addLap(void *stopwatch);
#endif

#if defined(TARGET_OS_OSX)
/**
 * A constructor that creates a new Stopwatch with default values.
 */
void *stopwatch_stopwatch_create(void);
#endif

#if defined(TARGET_OS_OSX)
/**
 * # Safety
 *
 * This function dereferences a raw pointer
 */
void *stopwatch_stopwatch_elapsedTime(void *stopwatch);
#endif

#if defined(TARGET_OS_OSX)
/**
 * # Safety
 *
 * This function dereferences a raw pointer and frees the object
 */
void stopwatch_stopwatch_free(void *stopwatch);
#endif

#if defined(TARGET_OS_OSX)
/**
 * # Safety
 *
 * This function dereferences a raw pointer
 */
void stopwatch_stopwatch_freeLapTimes(struct LapTimeBuffer buf);
#endif

#if defined(TARGET_OS_OSX)
/**
 * # Safety
 *
 * This function dereferences a raw pointer
 */
bool stopwatch_stopwatch_isRunning(void *stopwatch);
#endif

#if defined(TARGET_OS_OSX)
/**
 * # Safety
 *
 * This function dereferences a raw pointer
 */
struct LapTimeBuffer stopwatch_stopwatch_lapTimes(void *stopwatch);
#endif

#if defined(TARGET_OS_OSX)
/**
 * # Safety
 *
 * This function dereferences a raw pointer
 */
void stopwatch_stopwatch_reset(void *stopwatch);
#endif

#if defined(TARGET_OS_OSX)
/**
 * # Safety
 *
 * This function dereferences a raw pointer
 */
void stopwatch_stopwatch_start(void *stopwatch);
#endif

#if defined(TARGET_OS_OSX)
/**
 * # Safety
 *
 * This function dereferences a raw pointer
 */
void stopwatch_stopwatch_stop(void *stopwatch);
#endif
