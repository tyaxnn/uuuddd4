@echo off
echo Installing required packages...
pip install -r requirements_contour.txt

echo Running contour visualization...
python visualize_contour.py

pause