# ASCII converter

 Idea of this project is to make a program that converts image to ASCII

 Here are some ideas from google how this could be done
 1.  The logic is very simple. Move image to canvas, calculate and convert each pixel from RGB to grey scale, for each pixel calculate it's respective charactor value based on the intensity, sub-sampling of pixels. And done.  (https://www.reddit.com/r/webdev/comments/sr4grb/i_built_a_tool_to_convert_images_to_ascii_art/)
 2. (https://www.geeksforgeeks.org/converting-image-ascii-image-python/)
    1. Convert the input image to grayscale. 
    2. Split the image into M×N tiles. 
    3. Correct M (the number of rows) to match the image and font aspect ratio. 
    4. Compute the average brightness for each image tile and then look up a suitable ASCII character for each. 
    5. Assemble rows of ASCII character strings and print them to a file to form the final image.
 
## Possible techs
https://github.com/silvia-odwyer/photon
https://lib.rs/crates/imgref
https://stackoverflow.com/questions/27136950/getting-the-brightness-of-a-grayscale-pixel
