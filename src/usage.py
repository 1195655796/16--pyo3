import sys
import os

# 将当前目录添加到 sys.path
sys.path.append(os.path.dirname(os.path.abspath(__file__)))
import roaring_bitmap_py

bitmap = roaring_bitmap_py.RoaringBitmapWrapper()
bitmap.add(1)
print(bitmap.contains(1))  # 应该输出: True
print(bitmap.len())   