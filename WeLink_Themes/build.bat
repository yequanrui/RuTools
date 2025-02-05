cargo build --release
set "source=target\release\welink_themes.exe"
..\MT\mt.exe -manifest ..\MT\app.manifest -outputresource:%source%;#1
xcopy "%source%" "..\dist\" /c /h /i /r /y
set "target=..\..\..\Web\WeTools\resources\"
if exist %target% xcopy "%source%" "%target%" /c /h /i /r /y
