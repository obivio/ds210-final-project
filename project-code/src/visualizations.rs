use plotters::prelude::*;
use rand::Rng;

pub fn visualize_sampled_graph(graph: &[(usize, usize)], sampled_nodes: &[usize]) -> Result<(), Box<dyn std::error::Error>> {
    let root: DrawingArea<BitMapBackend<'_>, plotters::coord::Shift> = BitMapBackend::new("graph.png", (800, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .build_cartesian_2d(0..800, 0..800)?;

    let mut rng = rand::thread_rng();
    let mut positions = std::collections::HashMap::new();

    for &node in sampled_nodes {
        let x = rng.gen_range(50..750);
        let y = rng.gen_range(50..750);
        positions.insert(node, (x, y));
        chart.draw_series(std::iter::once(Circle::new((x, y), 5, &RED)))?;
        chart.draw_series(std::iter::once(Text::new(format!("{}", node), (x + 5, y + 5), ("sans-serif", 12).into_font())))?;
    }

    for &(src, dest) in graph {
        if let (Some(&start), Some(&end)) = (positions.get(&src), positions.get(&dest)) {
            chart.draw_series(std::iter::once(PathElement::new(vec![start, end], &BLUE)))?;
        }
    }

    root.present()?;
    Ok(())
}