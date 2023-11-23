

convert to wav :

	sox -t s16 -c 2 -r 44100 DEFAULT_60.pcm DEFAULT_60.WAV

convert all to wav :

	ls | awk -F "." '{print "sox -t s16 -c 2 -r 44100 " $1 ".pcm " $1 ".WAV" }' | bash
		
