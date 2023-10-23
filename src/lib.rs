pub mod node;
pub mod tree;
pub mod taxa;
pub mod iter;


#[cfg(test)]
mod tests {
    use crate::tree::{RootedPhyloTree, simple_rtree::SimpleRTree};
    #[test]
    fn read_small_tree() {
        let input_str = String::from("((A,B),(C,D));");
        let tree = RootedPhyloTree::from_newick(input_str);
        dbg!(tree.to_newick());
        }
    #[test]
    fn read_big_tree() {
    let input_str = String::from("(0,(1,(2,(3,(4,(5,(6,(7,(8,(9,(10,(11,(12,(13,(14,(15,(16,(17,(18,(19,(20,(21,(22,(23,(24,(25,(26,(27,(28,(29,(30,(31,(32,(33,(34,(35,(36,(37,(38,(39,(40,(41,(42,(43,(44,(45,(46,(47,(48,(49,(50,(51,(52,(53,(54,(55,(56,(57,(58,(59,(60,(61,(62,(63,(64,(65,(66,(67,(68,(69,(70,(71,(72,(73,(74,(75,(76,(77,(78,(79,(80,(81,(82,(83,(84,(85,(86,(87,(88,(89,(90,(91,(92,(93,(94,(95,(96, (97,98))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))");
    let tree = RootedPhyloTree::from_newick(input_str);
    dbg!(tree.to_newick());
    }
    #[test]
    fn read_smalllw_tree() {
        let input_str = String::from("((A:0.12,B:12),(C:10,D:0.001));");
        let tree = RootedPhyloTree::from_newick(input_str);
        dbg!(tree.to_newick());
        }
    #[test]
    fn read_smallfw_tree() {
        let input_str = String::from("((A:0.12,B:12):10,(C:15,D:0.001):20);");
        let tree = RootedPhyloTree::from_newick(input_str);
        dbg!(tree.to_newick());
        }
    #[test]
    fn read_smallfwfl_tree() {
        let input_str = String::from("((A:0.12,B:12)E:10,(C:15,D:0.001)F:20)G;");
        let tree = RootedPhyloTree::from_newick(input_str);
        dbg!(tree.to_newick());
        }
}
