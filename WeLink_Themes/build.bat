cargo build --release
set "source=target\release\welink_themes.exe"
..\MT\mt.exe -manifest ..\MT\app.manifest -outputresource:%source%;#1
xcopy "%source%" "..\dist\" /c /h /i /r /y
