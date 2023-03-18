use plotters::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use super::AccessDistr;

pub fn plot(plot_data : HashMap::<AccessDistr, Vec<f64>>, test_nums: Vec<usize>) -> Result<(), Box<dyn std::error::Error>> {
    let distrs : HashSet<String> = plot_data.keys().map(|x| x.distr.clone()).collect();
    let distrs : HashSet<(usize, String)> = distrs.into_iter().enumerate().collect();

    let accesses : HashSet<String> = plot_data.keys().map(|x| x.access.clone()).collect();
    let accesses : HashSet<(usize, String)> = accesses.into_iter().enumerate().collect();

    let min_x_value = test_nums[0];
    let max_x_value = test_nums[test_nums.len() - 1];
    let x_values_len = test_nums.len();
    let colors = vec![BLUE, RED, GREEN, YELLOW, CYAN, MAGENTA];
    let colors_len = colors.len();
    let key_points = test_nums.iter().map(|x| *x as f32).collect::<Vec<f32>>();

    for (_, access) in accesses.iter() {
        
        let out_file_name = format!("data/{}.png", access);
        let caption = format!("{} access", access);
        let root = BitMapBackend::new(&out_file_name, (1024, 768)).into_drawing_area();
        root.fill(&WHITE)?;

        let max_y = 
            plot_data
            .iter()
            .filter(|(k, _)| k.access == *access)
            .map(|(_, v)| (*v).iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap())
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();

        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(35)
            .y_label_area_size(40)
            .right_y_label_area_size(40)
            .margin(5)
            .caption(caption, ("sans-serif", 50.0).into_font())
            .build_cartesian_2d(
                ((min_x_value - 1) as f32..(max_x_value + 1) as f32).log_scale().with_key_points(key_points.clone()), 
                0.0f32..(*max_y) as f32
            )?;

        chart
            .configure_mesh()
            .disable_x_mesh()
            .disable_y_mesh()
            .y_desc("Average num of steps")
            .draw()?;
       
        for (idx, distr) in distrs.iter() {
            let y_data = plot_data.get(&AccessDistr { access: access.clone(), distr: distr.clone() }).unwrap();
            let label = format!("{} distribution", distr);

            chart
            .draw_series(LineSeries::new(
                (0..x_values_len).map(|x| (test_nums[x] as f32, y_data[x] as f32)),
                (&colors[*idx % colors_len]).stroke_width(3),
            ))?
            .label(label)
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &colors[*idx % colors_len]));
        }
        

        

        chart
            .configure_series_labels()
            .background_style(&RGBColor(128, 128, 128))
            .draw()?;

        // To avoid the IO failure being ignored silently, we manually call the present function
        root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
        println!("Result has been saved to {}", out_file_name);
    }

    for (_, distr) in distrs.iter() {
        let out_file_name = format!("data/{}.png", distr);
        let caption = format!("{} distribution", distr);
        let root = BitMapBackend::new(&out_file_name, (1024, 768)).into_drawing_area();
        root.fill(&WHITE)?;

        let max_y = 
            plot_data
            .iter()
            .filter(|(k, _)| k.distr == *distr)
            .map(|(_, v)| (*v).iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap())
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();

        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(35)
            .y_label_area_size(40)
            .right_y_label_area_size(40)
            .margin(5)
            .caption(caption, ("sans-serif", 50.0).into_font())
            .build_cartesian_2d(
                ((min_x_value - 1) as f32..(max_x_value + 1) as f32).log_scale().with_key_points(key_points.clone()), 
                0.0f32..(*max_y) as f32
            )?;

        chart
            .configure_mesh()
            .disable_x_mesh()
            .disable_y_mesh()
            .y_desc("Average num of steps")
            .draw()?;
       
        for (idx, access) in accesses.iter() {
            let y_data = plot_data.get(&AccessDistr { access: access.clone(), distr: distr.clone() }).unwrap();
            let label = format!("{} access", access);

            chart
            .draw_series(LineSeries::new(
                (0..x_values_len).map(|x| (test_nums[x] as f32, y_data[x] as f32)),
                (&colors[*idx % colors_len]).stroke_width(3),
            ))?
            .label(label)
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &colors[*idx % colors_len]));
        }
        

        

        chart
            .configure_series_labels()
            .background_style(&RGBColor(128, 128, 128))
            .draw()?;

        // To avoid the IO failure being ignored silently, we manually call the present function
        root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
        println!("Result has been saved to {}", out_file_name);
    }

    Ok(())
}