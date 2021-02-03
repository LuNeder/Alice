# Alice
Send the contnents of a file in a chat, each line on a message

### Usage

- Download/clone this repository 
- Open terminal and cd to the directory original.txt is (usually the same directory as alice.py is)
- Type/paste whatever you want in original.txt
- Run on terminal: "python /PATH/TO/alice.py" (change /PATH/TO/alice.py to the alice.py path. If you are already on alice.py's directory and alice.py and original.txt are in the same directory, you can just type ./alice.py instead)
- Type the number of seconds you want between a line and the next one and press enter
- Select the text box/input area you want Alice to type in
- After 10 seconds, alice will start typing

#### Important notes
Alice requires an original.txt file on the directory you're running it from.

Alice will read the file and then type it (a line followed by an enter, then the next line).

This way you can send the file contnent on a chat, for example, each line on a message.

You can also tell Alice to wait some time between a line and the other.

The first line will be written 10 seconds after you run Alice and tell the time between a line and the other, this way you have time to select the place where you want to input your text.

"Alice" was just the 1st name I found on Google lol

### Dependencies 
- keyboard (pip install keyboard)
- time (pip install time)
- python 3.9 or newer
