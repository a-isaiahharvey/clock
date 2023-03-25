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
  /**
   * A pointer to the data stored in the buffer.
   */
  const void *data;
  /**
   * The length of the buffer.
   */
  size_t len;
} LapTimeBuffer;
#endif

#if defined(TARGET_OS_OSX)
/**
 * # Safety
 *
 * This function dereferences a raw pointer
 */
uint64_t rust_Duration_asSecs(const void *duration);
#endif

#if defined(TARGET_OS_OSX)
/**
 * # Safety
 *
 * This function dereferences a raw pointer
 */
double rust_Duration_asSecsF64(const void *duration);
#endif

#if defined(TARGET_OS_OSX)
/**
 * # Safety
 *
 * This function dereferences a raw pointer
 */
bool rust_Duration_eq(const void *duration, const void *other);
#endif

#if defined(TARGET_OS_OSX)
/**
 * # Safety
 *
 * This function dereferences a raw pointer
 */
void rust_Duration_free(void *duration);
#endif

#if defined(TARGET_OS_OSX)
/**
 * # Safety
 *
 * This function dereferences a raw pointer
 */
bool rust_Duration_isZero(const void *duration);
#endif

#if defined(TARGET_OS_OSX)
/**
 * # Safety
 *
 * This function dereferences a raw pointer
 */
void stopwatch_LapTime_free(void *laptime);
#endif

#if defined(TARGET_OS_OSX)
/**
 * # Safety
 *
 * This function dereferences a raw pointer
 */
void *stopwatch_LapTime_getIndex(struct LapTimeBuffer buf, size_t index);
#endif

#if defined(TARGET_OS_OSX)
/**
 * # Safety
 *
 * This function dereferences a raw pointer
 */
size_t stopwatch_LapTime_lapNumber(void *laptime);
#endif

#if defined(TARGET_OS_OSX)
/**
 * # Safety
 *
 * This function dereferences a raw pointer
 */
void *stopwatch_LapTime_splitTime(void *laptime);
#endif

#if defined(TARGET_OS_OSX)
/**
 * # Safety
 *
 * This function dereferences a raw pointer
 */
void *stopwatch_LapTime_totalTime(void *laptime);
#endif

#if defined(TARGET_OS_OSX)
/**
 * # Safety
 *
 * This function dereferences a raw pointer
 */
void stopwatch_Stopwatch_addLap(void *stopwatch);
#endif

#if defined(TARGET_OS_OSX)
/**
 * A constructor that creates a new Stopwatch with default values.
 */
void *stopwatch_Stopwatch_create(void);
#endif

#if defined(TARGET_OS_OSX)
/**
 * # Safety
 *
 * This function dereferences a raw pointer
 */
void *stopwatch_Stopwatch_elapsedTime(void *stopwatch);
#endif

#if defined(TARGET_OS_OSX)
/**
 * # Safety
 *
 * This function dereferences a raw pointer and frees the object
 */
void stopwatch_Stopwatch_free(void *stopwatch);
#endif

#if defined(TARGET_OS_OSX)
/**
 * # Safety
 *
 * This function dereferences a raw pointer
 */
void stopwatch_Stopwatch_freeLapTimes(struct LapTimeBuffer buf);
#endif

#if defined(TARGET_OS_OSX)
/**
 * # Safety
 *
 * This function dereferences a raw pointer
 */
bool stopwatch_Stopwatch_isRunning(void *stopwatch);
#endif

#if defined(TARGET_OS_OSX)
/**
 * # Safety
 *
 * This function dereferences a raw pointer
 */
struct LapTimeBuffer stopwatch_Stopwatch_lapTimes(void *stopwatch);
#endif

#if defined(TARGET_OS_OSX)
/**
 * # Safety
 *
 * This function dereferences a raw pointer
 */
void stopwatch_Stopwatch_reset(void *stopwatch);
#endif

#if defined(TARGET_OS_OSX)
/**
 * # Safety
 *
 * This function dereferences a raw pointer
 */
void stopwatch_Stopwatch_start(void *stopwatch);
#endif

#if defined(TARGET_OS_OSX)
/**
 * # Safety
 *
 * This function dereferences a raw pointer
 */
void stopwatch_Stopwatch_stop(void *stopwatch);
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
void *timer_Timer_create(uint64_t secs);
#endif

#if defined(TARGET_OS_OSX)
/**
 * # Safety
 *
 * This function dereferences a raw pointer
 */
void *timer_Timer_duration(void *timer);
#endif

#if defined(TARGET_OS_OSX)
/**
 * # Safety
 *
 * This function dereferences a raw pointer
 */
void *timer_Timer_elapsed(void *timer);
#endif

#if defined(TARGET_OS_OSX)
/**
 * # Safety
 *
 * This function dereferences a raw pointer
 */
bool timer_Timer_eq(void *timer, void *other);
#endif

#if defined(TARGET_OS_OSX)
/**
 * # Safety
 *
 * This function dereferences a raw pointer and frees the object
 */
void timer_Timer_free(void *timer);
#endif

#if defined(TARGET_OS_OSX)
/**
 * # Safety
 *
 * This function dereferences a raw pointer
 */
bool timer_Timer_hasNotStarted(void *timer);
#endif

#if defined(TARGET_OS_OSX)
/**
 * # Safety
 *
 * This function dereferences a raw pointer
 */
bool timer_Timer_isDone(void *timer);
#endif

#if defined(TARGET_OS_OSX)
/**
 * # Safety
 *
 * This function dereferences a raw pointer
 */
bool timer_Timer_isRunning(void *timer);
#endif

#if defined(TARGET_OS_OSX)
/**
 * # Safety
 *
 * This function dereferences a raw pointer
 */
void *timer_Timer_remaining(void *timer);
#endif

#if defined(TARGET_OS_OSX)
/**
 * # Safety
 *
 * This function dereferences a raw pointer
 */
void timer_Timer_reset(void *timer);
#endif

#if defined(TARGET_OS_OSX)
/**
 * # Safety
 *
 * This function dereferences a raw pointer
 */
void timer_Timer_start(void *timer);
#endif

#if defined(TARGET_OS_OSX)
/**
 * # Safety
 *
 * This function dereferences a raw pointer
 */
void timer_Timer_stop(void *timer);
#endif
