#import <Foundation/Foundation.h>
#import <GameController/GameController.h>
#import <CoreHaptics/CoreHaptics.h>

// Called only on the game's main thread. ARC retains the engine and current pulse.
static GCController *device;
static CHHapticEngine *engine;
static id<CHHapticPatternPlayer> player;
void arcade_haptics_stop(void) {
    @autoreleasepool { [player stopAtTime:CHHapticTimeImmediate error:NULL]; player = nil; }
}
void arcade_haptics_shutdown(void) {
    @autoreleasepool {
        arcade_haptics_stop();
        [engine stopWithCompletionHandler:nil];
        engine = nil;
        device = nil;
    }
}
int arcade_haptics_status(void) {
    @autoreleasepool {
        NSArray<GCController *> *devices = GCController.controllers;
        // GLFW and GameController indices aren't interchangeable. Restrict to a
        // single controller so we never vibrate an unrelated player's device.
        GCController *selected = devices.count == 1 ? devices.firstObject : nil;
        if (selected != device) arcade_haptics_shutdown();
        if (!selected || ![selected.haptics.supportedLocalities containsObject:GCHapticsLocalityDefault]) return 0;
        if (!engine) {
            device = selected;
            engine = [device.haptics createEngineWithLocality:GCHapticsLocalityDefault];
            engine.autoShutdownEnabled = YES;
        }
        return engine != nil;
    }
}
int arcade_haptics_play(float intensity, float sharpness, float duration) {
    @autoreleasepool {
        if (!arcade_haptics_status()) return 0;
        NSError *error = nil;
        if (![engine startAndReturnError:&error]) { NSLog(@"Arcade haptics: %@", error.localizedDescription); return 0; }
        arcade_haptics_stop();
        CHHapticEventParameter *power = [[CHHapticEventParameter alloc] initWithParameterID:CHHapticEventParameterIDHapticIntensity value:fminf(1, fmaxf(0, intensity))];
        CHHapticEventParameter *texture = [[CHHapticEventParameter alloc] initWithParameterID:CHHapticEventParameterIDHapticSharpness value:fminf(1, fmaxf(0, sharpness))];
        CHHapticEvent *event = [[CHHapticEvent alloc] initWithEventType:CHHapticEventTypeHapticContinuous parameters:@[power, texture] relativeTime:0 duration:fminf(0.7, fmaxf(0.01, duration))];
        CHHapticPattern *pattern = [[CHHapticPattern alloc] initWithEvents:@[event] parameters:@[] error:&error];
        if (!pattern) return 0;
        player = [engine createPlayerWithPattern:pattern error:&error];
        BOOL played = player && [player startAtTime:CHHapticTimeImmediate error:&error];
        if (!played) NSLog(@"Arcade haptics: %@", error.localizedDescription);
        return played;
    }
}
