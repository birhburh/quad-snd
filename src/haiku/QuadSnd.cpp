#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <Entry.h>
#include <InterfaceDefs.h>
#include <MediaFile.h>
#include <MediaTrack.h>
#include <PushGameSound.h>

extern "C" {
    BPushGameSound* push_game_sound_create_start(float frame_rate, uint32 channel_count, size_t framesPerBufferPart, size_t bufferPartCount)
    {
        gs_audio_format gsFormat;
        memset(&gsFormat, 0, sizeof(gsFormat));
        gsFormat.frame_rate = frame_rate;
        gsFormat.channel_count = channel_count;
        gsFormat.format = gs_audio_format::B_GS_S16;
        gsFormat.byte_order = B_MEDIA_LITTLE_ENDIAN;
        gsFormat.buffer_size = framesPerBufferPart;

        BPushGameSound* pushGameSound = new BPushGameSound(framesPerBufferPart, &gsFormat, bufferPartCount);
        
        if (pushGameSound->InitCheck() != B_OK) {
            printf("trouble initializing push game sound: %s\n", strerror(pushGameSound->InitCheck()));        
        }
        
        if (pushGameSound->StartPlaying() != B_OK) {
            printf("cannot start playback\n");
        }

        return pushGameSound;
    }

    void lock_next_page(BPushGameSound *pushGameSound, void** outPagePtr, size_t* outPageSize)
    {
        if (pushGameSound->LockNextPage(outPagePtr, outPageSize) != BPushGameSound::lock_ok) {
            printf("cannot lock page\n");
            return;
        }
    }

    void unlock_page(BPushGameSound *pushGameSound, void* inPagePtr)
    {
        if (pushGameSound->UnlockPage(inPagePtr) != B_OK)
        {
            printf("cannot unlock page\n");
            return;
        }
    }

    void push_game_sound_stop(BPushGameSound *pushGameSound)
    {
        pushGameSound->StopPlaying();
    }
}



