use std::fmt::{Debug, Display};
use std::hash::Hash;

use itertools::Itertools;
use num::{Num, NumCast};

pub trait RootedTreeNode
where
    Self: Clone,
{    
    type NodeID: Display + Debug + Hash + Clone + Drop + Ord + PartialEq + Eq;

    fn new(id: Self::NodeID)->Self;
    fn get_id(&self)->Self::NodeID;
    fn set_id(&mut self, id: Self::NodeID);
    fn get_parent(&self)->Option<Self::NodeID>;
    fn set_parent(&mut self, parent: Option<Self::NodeID>);
    fn get_children(&self)->impl IntoIterator<Item=Self::NodeID, IntoIter = impl ExactSizeIterator<Item = Self::NodeID>>;
    fn add_child(&mut self, child:Self::NodeID);
    fn remove_child(&mut self, child:&Self::NodeID);

    fn is_leaf(&self)->bool
    {
        self.get_children().into_iter().next().is_none()
    }


    fn node_type(&self)->String{
        match self.is_leaf() {
            false => "Internal".to_string(),
            true => "Leaf".to_string(),
        }
    }
    fn add_children(&mut self, children: impl IntoIterator<Item=Self::NodeID>){
        for child in children.into_iter(){
            self.add_child(child);
        }
    }
    fn remove_children(&mut self, children: impl IntoIterator<Item=Self::NodeID>){
        for child in children.into_iter(){
            self.remove_child(&child);
        }
    }
    fn num_children(&self)->usize
    {
        self.get_children().into_iter().collect::<Vec<Self::NodeID>>().len()
    }
    fn degree(&self)->usize
    {
        match self.get_parent()
        {
            Some(_) => self.num_children()+1,
            None => self.num_children()
        }
    }
    fn neighbours(&self)->impl IntoIterator<Item=Self::NodeID, IntoIter = impl ExactSizeIterator<Item = Self::NodeID>>
    {
        let mut children = self.get_children().into_iter().collect_vec();
        match self.get_parent(){
            Some(p) => {children.push(p);},
            None => {},
        }
        children
    }
}

pub trait RootedMetaNode: RootedTreeNode
{
    type Meta: Display + Debug + Eq + PartialEq + Clone + Ord;

    fn get_taxa(&self)->Option<Self::Meta>;
    fn set_taxa(&mut self, taxa: Option<Self::Meta>);

}

pub trait RootedWeightedNode: RootedTreeNode
{
    type Weight: Num + Clone + PartialOrd + NumCast + std::iter::Sum;
    
    fn get_weight(&self)->Option<Self::Weight>;
    fn set_weight(&mut self, w: Option<Self::Weight>);
    fn unweight(&mut self){
        self.set_weight(None);
    }
}
