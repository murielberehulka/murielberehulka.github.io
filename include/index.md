title=Muriel Berehulka
++header
++_base
for_each_folder=include/posts
>.folder
    >h4 {{title}}
for_each_md_in_current_folder
    >div
        >a href="{{url}}" {{title}}
;;;
;;;
