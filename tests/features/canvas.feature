Feature: Canvas feature

  Scenario: Creating a canvas
    Given c <- canvas[10, 20]
    Then c.width = 10
    And c.height = 20
    And every pixel of c is color[0.0, 0.0, 0.0]

@test
  Scenario: Writing pixels to a canvas
    Given c <- canvas[10, 20]
    And red <- color[1.0, 0.0, 0.0]
    When write_pixel[2, 3, red] 
    Then pixel_at[2, 3] = red


  Scenario: Constructing the PPM heeader
   Given c <- canvas[5, 3]
   When ppm <- to_ppm
   Then lines 1-3 of ppm are
   """
   P3
   5 3
   255
   """ 

   Scenario: Constructing the PPM pixel data
   Given c <- canvas[2, 2]
   And c1 <- color[1.5, 0.0, 0.0]
   And c2 <- color[0.0, 0.5, 0.0]
   And c3 <- color[-0.5, 0.0, 1] 
   When write_pixel[0, 0, c1]
   And write_pixel[1, 0, c2]
   And write_pixel[0, 1, c3]
   And ppm <- to_ppm
   Then lines 4-7 of ppm are
   """
   255 0 0
   0 128 0
   0 0 255
   0 0 0
   """
