include <Round-Anything/polyround.scad>

// Play with these values:
$fn = 256;          // model resolution
ring_id = 23;       // Inner Diameter
ring_thick = 3;
h = 26;
hinge_gap = .2;     // tolerance
latch_dist = 6;     // minimum 6, probably best left as is
latch_thick = 2;
latch_round = 4;
do_latch_round = true;

// Probably shouldn't touch:
ring_od = ring_id + ring_thick * 2;
angle = 20;
hinge_d = ring_id + 16;
hingeh = h + 2;
hinge_ri = 4;
hinge_ro = 7;

module ring(){
    // Ring
    difference(){
        in_r    = ring_id / 2;
        out_r   = in_r + ring_thick;
        
        cylinder(h, out_r, out_r);
        
        translate([0,0,-1]){
            cylinder(h+2, in_r, in_r);
        }
    }
}
    
    


module latch0(){
    
    mag_i = ring_id / 2 + ring_thick + latch_dist;
    mag_o = mag_i + latch_thick;
    
    
    r = 1000;
    r2 = .4;
    
    tooth = 3;
    
    v0x = cos(90+angle+4) * mag_o;
    v0y = sin(90+angle+5) * mag_o;
    v1x = cos(90+angle/2) * mag_o;
    v1y = sin(90+angle/2) * mag_o;
    v2x = 0;
    v2y = mag_o;
    v3x = cos(90-angle/2) * mag_o;
    v3y = sin(90-angle/2) * mag_o;
    v4x = cos(90-angle) * mag_o;
    v4y = sin(90-angle) * mag_o;
    v5x = cos(90-angle-35) * ring_od/2;
    v5y = sin(90-angle-35) * ring_od/2;
    v6x = cos(90-angle) * ring_id/2+ring_thick/2;
    v6y = sin(90-angle) * ring_id/2+ring_thick/2;
    v7x = cos(90) * ring_id/2;
    v7y = sin(90) * ring_id/2;
    v8x = cos(90) * (mag_i-.5);
    v8y = sin(90) * (mag_i-.5);
    v9x = cos(90-angle+15) * (mag_i-.5);
    v9y = sin(90-angle+15) * (mag_i-.5);
    v10x = cos(90-angle+9) * (mag_i-.5);
    v10y = sin(90-angle+9) * (mag_i-.5);
    v11x = cos(90-angle+5) * ring_od/2;
    v11y = sin(90-angle+5) * ring_od/2;
    v12x = cos(90-angle-10) * ring_od/2;
    v12y = sin(90-angle-10) * ring_od/2;
    v13x = cos(90-angle+5) * mag_i;
    v13y = sin(90-angle+5) * mag_i;
    v14x = cos(90-angle/2) * mag_i;
    v14y = sin(90-angle/2) * mag_i;
    v15x = 0;
    v15y = mag_i;
    v16x = cos(90+angle/2) * (mag_i);
    v16y = sin(90+angle/2) * (mag_i);
    v17x = cos(90+angle/2) * (mag_i - tooth);
    v17y = sin(90+angle/2) * (mag_i - tooth);
    v18x = cos(90+angle/2+8) * mag_i;
    v18y = sin(90+angle/2+8) * mag_i;
    v19x = cos(90+angle+5) * mag_i;
    v19y = sin(90+angle+5) * mag_o-(mag_o-mag_i)/2;

    
    points = [
        [v0x,v0y,0],
        [v1x,v1y,r],
        [v2x,v2y,r],
        [v3x,v3y,r],
        [v4x,v4y,r],
        [v5x,v5y,0],
        [v6x,v6y,0],
        [v7x,v7y,0],
        [v8x,v8y,r2],
        [v9x,v9y,r2],
        [v10x,v10y,r2],
        [v11x,v11y,10],
        [v12x,v12y,r],
        [v13x,v13y,r],
        [v14x,v14y,r],
        [v15x,v15y,r],
        [v16x,v16y,0],
        [v17x,v17y,r2],
        [v18x,v18y,r2],
        [v19x,v19y,r2]
    ];
    linear_extrude(h)polygon(polyRound(points,10));
      
}

module latch0_round(){
    r = latch_round;
    off = 2;
    rotate([90,0,180+angle]){
        points = [
        [off   ,0  ,r],
        [off   ,h  ,r],
        [off-r ,h  ,0],
        [off-r ,h+1,0],
        [20     ,h+1,0],
        [20     ,-1 ,0],
        [off-r ,-1 ,0],
        [off-r ,0  ,0]
        ];
        linear_extrude(100)polygon(polyRound(points,32));
    }
}

module latch1(){
    mag_i = ring_id / 2 + ring_thick + latch_dist;
    mag_o = mag_i + latch_thick;
    
    r = 1000;
    r2 = .4;
    
    tooth = 3;
    
    v0x = cos(90) * (mag_i - 2);
    v0y = sin(90) * (mag_i - 2);
    v1x = cos(90) * ring_id/2;
    v1y = sin(90) * ring_id/2;
    v2x = cos(90+angle/2) * (ring_id/2+ring_thick/2);
    v2y = sin(90+angle/2) * (ring_id/2+ring_thick/2);
    v3x = cos(90+angle+10) * (ring_id/2+ring_thick/2);
    v3y = sin(90+angle+10) * (ring_id/2+ring_thick/2);
    v4x = cos(90+angle) * ring_od/2;
    v4y = sin(90+angle) * ring_od/2;
    v5x = cos(90+angle-6) * (mag_i-tooth);
    v5y = sin(90+angle-6) * (mag_i-tooth);
    v6x = cos(90+angle/2) * (mag_i-tooth);
    v6y = sin(90+angle/2) * (mag_i-tooth);
    v7x = cos(90+angle/2) * (mag_i-1);
    v7y = sin(90+angle/2) * (mag_i-1);
    v8x = cos(90+angle/2-1) * (mag_i-1);
    v8y = sin(90+angle/2-1) * (mag_i-1);
    v9x = cos(90+angle/4) * (mag_i-1);
    v9y = sin(90+angle/4) * (mag_i-1);
    
    points = [
        [v0x,v0y,r2],
        [v1x,v1y,0],
        [v2x,v2y,0],
        [v3x,v3y,0],
        [v4x,v4y,r2],
        [v5x,v5y,r2],
        [v6x,v6y,r2],
        [v7x,v7y,r2],
        [v8x,v8y,0],
        [v9x,v9y,r],
    ];
    linear_extrude(h)polygon(polyRound(points,10));
    
}


module hinge_0(){
    ri = hinge_ri;
    ro = hinge_ro;
    difference(){
        cylinder(hingeh, ro, ro);
        translate([0,0,-1]){
            cylinder(hingeh+2, ri, ri);
        }
        rotate([90,0,0]){
            translate([0,0,-20]){
                points = [
                    [-ro,4,0],
                    [-ro,h,0],
                    [0,h-8,2],
                    [0,4,2],
                    ];
                linear_extrude(40)polygon(polyRound(points,10));
            }
        }
    }
}

module hinge_1(){

    ri = hinge_ri - hinge_gap;
    ro = hinge_ro + hinge_gap;
    
    cylinder(hingeh, ri, ri);

    rotate([90,0,0]){
        translate([0,0,-ro]){
            points = [
                [-ro/2,4+hinge_gap,2],
                [-ro/2,h-8,2],
                [-ro/4,h-6,1],
                [-hinge_gap,h-8,2],
                [-hinge_gap,4+hinge_gap,2],
                ];
            linear_extrude(ro)polygon(polyRound(points,10));
        }
    }
}
module part0(){
    color("grey"){
        translate([0,hinge_d/2,0]){
            difference(){
                union(){
                    difference(){
                        latch0();
                        if (do_latch_round) {
                            latch0_round();
                        }
                    }
                    difference(){
                        ring();
                
                        translate([-300,-150,-1]){
                            cube(300);
                        }
                    }
                }            
                translate([-1,ring_od/2+1,h/2]){
                    rotate([0,90,0]){
                        cylinder(3,4,0, $fn = 4);
                    }
                }
            }
        }
        hinge_0();
    }
}

module body_attach(){
    cube(20);
}
module part1(){
    rotate([0,0,0]){
        difference(){
            translate([0,hinge_d/2 ,0]){
                color("darkgrey"){
                    latch1();
                    ring();
                    translate([-5,0,h/2]){
                        cube([26,ring_id,h],center=true);
                    }
                }
            }
            
            // Ring inside carve
            translate([0,hinge_d/2,-1]){
                cylinder(h+2, ring_id / 2, ring_id / 2);
            }
            // Hinge outside carve
            cylinder(h+2, hinge_ro + hinge_gap, hinge_ro + hinge_gap);
            // Ring Half carve 
            translate([-hinge_gap,-150,-1]){
                cube(300);
            }
            // Screw hole
            translate([0,hinge_d/2,h/2]){
                rotate([0,-90,0]){
                    // screwhead
                    cylinder(ring_id/2+2,5,5);
                    // body
                    translate([0,0,ring_id/2+3]){
                        cylinder(ring_id/2+2,5,5);
                    }
                    // hole
                    cylinder(ring_id,2.5,2.5);
                }
            }
            translate([-hinge_d-16,hinge_d/2,-1]){
                cylinder(h+2,hinge_d,hinge_d);
            }
        }
        hinge_1();
        translate([0,hinge_d/2 + ring_od/2+1,h/2]){
            rotate([0,90,0]){
                cylinder(2-hinge_gap,2/.75-hinge_gap,0, $fn = 4);
            }
        }
    }
}


part0();
part1();
//body_attach();
