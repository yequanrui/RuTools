cargo build --release
set "target=..\..\..\Web\WeTools\resources\"
set "source=target\release\welink_msg_style.exe"
if exist %source% (
..\MT\mt.exe -manifest ..\MT\app.manifest -outputresource:%source%;#1
xcopy "%source%" "..\dist\" /c /h /i /r /y
if exist %target% xcopy "%source%" "%target%" /c /h /i /r /y
)
set "source_blue=target\release\welink_msg_style_blue.exe"
if exist %source_blue% (
..\MT\mt.exe -manifest ..\MT\app.manifest -outputresource:%source_blue%;#1
xcopy "%source_blue%" "..\dist\" /c /h /i /r /y
if exist %target% xcopy "%source_blue%" "%target%" /c /h /i /r /y
)
