def get_x_coords():
    x_coords = []
    
    with open('../data/max_steps.txt', 'r') as f:
        for line in f:
            x_coords.append(int(line))
            
    return x_coords

def get_y_coords():
    y_coords = []
    
    with open('../data/max_step_gap.txt', 'r') as f:
        for line in f:
            y_coords.append(int(line))
            
    return y_coords