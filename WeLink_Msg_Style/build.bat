cargo build --release
set "source=target\release\weLink_msg_style.exe"
..\MT\mt.exe -manifest ..\MT\app.manifest -outputresource:%source%;#1
xcopy "%source%" "..\dist\" /c /h /i /r /y
