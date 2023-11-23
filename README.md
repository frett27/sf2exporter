
# Command line to export a sf2 principal Bank to WAV files

This command line export each notes of the sf2 soundfont into a specific sound file (either .pcm or WAV)

Commandline description :


```
Parameters definition for rendering

Usage: sf2exporter --sf2 <SF2> --output <OUTPUT>

Options:
  -s, --sf2 <SF2>        sf2 filepath
  -o, --output <OUTPUT>  output format (wav or pcm)
  -h, --help             Print help
  -V, --version          Print version


```



## additional tools commands

convert to wav :

	sox -t s16 -c 2 -r 44100 DEFAULT_60.pcm DEFAULT_60.WAV

convert all to wav :

	ls | awk -F "." '{print "sox -t s16 -c 2 -r 44100 " $1 ".pcm " $1 ".WAV" }' | bash
		
